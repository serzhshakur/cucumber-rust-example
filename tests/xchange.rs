use std::{collections::HashMap, convert::Infallible};

use anyhow::bail;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use cucumber::{given, then, when, World, WorldInit};
use xchange_cucumber_rust::{
    api::api::{MarketApi, UserApi},
    requests::{AssetPairInfo, AssetPairsRequest},
    responses::{AssetPairResponse, OpenOrderResponse, ServerTimeResponse},
    Deps,
};

#[derive(Debug)]
enum State {
    Empty,
    ServerTime(ServerTimeResponse),
    OpenOrders(OpenOrderResponse),
    AssetPair(String, Option<HashMap<String, AssetPairResponse>>),
}

#[derive(WorldInit, Debug)]
pub struct MyWorld {
    deps: Deps,
    data: State,
}

#[async_trait(?Send)]
impl World for MyWorld {
    type Error = Infallible;

    async fn new() -> Result<Self, Infallible> {
        let deps = Deps::init().await;
        Ok(Self {
            deps,
            data: State::Empty,
        })
    }
}

#[when("server time is retrieved")]
async fn retrieve_server_time(world: &mut MyWorld) -> anyhow::Result<()> {
    let res = world.deps.api.get_server_time().await?;
    world.data = State::ServerTime(res.unwrap());
    Ok(())
}

#[then("both unixtime and rfc1123 are returned in UTC format")]
async fn check_server_time(world: &mut MyWorld) {
    match &world.data {
        State::ServerTime(data) => {
            let now = Utc::now();
            let unix_time = data.unixtime;
            let rfc_time = DateTime::parse_from_rfc2822(&data.rfc1123).unwrap();

            assert!(now.signed_duration_since(unix_time).num_seconds() < 2);
            assert!(now.signed_duration_since(rfc_time).num_seconds() < 2);
        }
        _ => panic!("server time is not retrieved"),
    }
}

#[given(regex = r#"^client chooses asset pair "([0-9A-Za-z]+)/([0-9A-Za-z]+)"$"#)]
async fn set_asset_pair(
    world: &mut MyWorld,
    base_asset: String,
    quote_asset: String,
) -> anyhow::Result<()> {
    let pair = format!("{}{}", base_asset.trim(), quote_asset.trim());
    world.data = State::AssetPair(pair, None);
    Ok(())
}

#[when("an asset pair is queried")]
async fn query_asset_pair(world: &mut MyWorld) -> anyhow::Result<()> {
    match &world.data {
        State::AssetPair(pair, _) => {
            let req = AssetPairsRequest {
                pair: pair.to_owned(),
                info: Some(AssetPairInfo::Info),
            };
            let res = world.deps.api.get_asset_pairs(req).await?;
            world.data = State::AssetPair(pair.to_owned(), Some(res));
            Ok(())
        }
        _ => bail!("no pair is set"),
    }
}

#[then("a valid info about asset pair is returned")]
async fn check_asset_pairs_response(world: &mut MyWorld) -> anyhow::Result<()> {
    match &world.data {
        State::AssetPair(pair, res) => match res {
            Some(res) => {
                assert_eq!(res.len(), 1);
                assert!(res.contains_key(pair));
            }
            None => bail!("no API response is set in previous step"),
        },
        _ => bail!("no asset pair is set"),
    }
    Ok(())
}

#[when("I query open orders")]
async fn query_open_orders(world: &mut MyWorld) -> anyhow::Result<()> {
    let res = world.deps.api.get_open_orders().await?;
    world.data = State::OpenOrders(res);
    Ok(())
}

#[then(regex = r"^number of open orders is (\d+)$")]
async fn check_open_orders_count(world: &mut MyWorld, count: u8) -> anyhow::Result<()> {
    match &world.data {
        State::OpenOrders(orders) => {
            assert_eq!(count, orders.open.len() as u8);
            Ok(())
        }
        _ => bail!("open orders are not queried"),
    }
}

#[tokio::main]
async fn main() {
    MyWorld::cucumber().run_and_exit("tests/features/").await;
    // AnimalWorld::run("tests/features/").await;
}
