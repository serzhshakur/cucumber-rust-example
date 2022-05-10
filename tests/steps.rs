use std::{collections::HashMap, convert::Infallible, str::FromStr};

use anyhow::bail;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use cucumber::{given, then, when, Parameter, World, WorldInit};
use xchange_cucumber_rust::{
    api::api::{MarketApi, UserApi},
    requests::AssetPairsRequest,
    responses::{AssetPairResponse, OpenOrderResponse, ServerTimeResponse},
    Deps,
};

#[derive(Debug)]
enum State {
    Empty,
    ServerTime(ServerTimeResponse),
    OpenOrders(OpenOrderResponse),
    AssetPair(Pair, Option<HashMap<String, AssetPairResponse>>),
}

#[derive(Debug, Eq, Parameter, PartialEq, Clone)]
#[param(name = "pair", regex = "([0-9A-Za-z]+/[0-9A-Za-z]+)")]
struct Pair {
    name: String,
    base: String,
    quote: String,
    altname: String,
    wsname: String,
}

impl FromStr for Pair {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Pair> {
        let split: Vec<&str> = s.split("/").collect();
        if split.len() != 2 {
            bail!("Asset pair must be in the format BASE/QUOTE");
        }
        let base = split[0];
        let quote = split[1];

        fn alt_name(s: &str) -> &str {
            let has_asset_code = s.len() > 3 && (s.starts_with("X") || s.starts_with("Z"));
            if has_asset_code {
                &s[1..]
            } else {
                s
            }
        }

        let alt_base = alt_name(base);
        let alt_quote = alt_name(quote);

        let asset_pair = Pair {
            name: format!("{}{}", base, quote),
            base: base.to_string(),
            quote: quote.to_string(),
            altname: format!("{}{}", alt_base, alt_quote),
            wsname: format!("{}/{}", alt_base, alt_quote),
        };

        Ok(asset_pair)
    }
}

#[derive(WorldInit, Debug)]
pub struct MyWorld {
    deps: Deps,
    state: State,
}

#[async_trait(?Send)]
impl World for MyWorld {
    type Error = Infallible;

    async fn new() -> Result<Self, Infallible> {
        let deps = Deps::init().await;
        Ok(Self {
            deps,
            state: State::Empty,
        })
    }
}

#[when("server time is retrieved")]
async fn retrieve_server_time(world: &mut MyWorld) -> anyhow::Result<()> {
    let res = world.deps.api.get_server_time().await?;
    world.state = State::ServerTime(res.unwrap());
    Ok(())
}

#[then("both unixtime and rfc1123 are returned in UTC format")]
async fn check_server_time(world: &mut MyWorld) {
    match &world.state {
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

#[given(expr = "client chooses asset pair \"{pair}\"")]
async fn set_asset_pair(world: &mut MyWorld, asset_pair: Pair) -> anyhow::Result<()> {
    world.state = State::AssetPair(asset_pair, None);
    Ok(())
}

#[when("an asset pair is queried")]
async fn query_asset_pair(world: &mut MyWorld) -> anyhow::Result<()> {
    match &world.state {
        State::AssetPair(pair, _) => {
            let req = AssetPairsRequest {
                pair: pair.name.to_owned(),
                info: None,
            };
            let res = world.deps.api.get_asset_pairs(req).await?;
            world.state = State::AssetPair(pair.clone(), Some(res));
            Ok(())
        }
        _ => bail!("no pair is set"),
    }
}

#[then("a valid info about asset pair is returned")]
async fn check_asset_pairs_response(world: &mut MyWorld) -> anyhow::Result<()> {
    match &world.state {
        State::AssetPair(pair, res) => match res {
            Some(res) => {
                assert_eq!(res.len(), 1);
                assert!(res.contains_key(&pair.name));

                let res = res.get(&pair.name).unwrap();
                assert_eq!(res.altname, pair.altname);
                assert_eq!(res.wsname, pair.wsname);
                assert_eq!(res.quote, pair.quote);
                assert_eq!(res.base, pair.base);
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
    world.state = State::OpenOrders(res);
    Ok(())
}

#[then(regex = r"^number of open orders is (\d+)$")]
async fn check_open_orders_count(world: &mut MyWorld, count: u8) -> anyhow::Result<()> {
    match &world.state {
        State::OpenOrders(orders) => {
            assert_eq!(count, orders.open.len() as u8);
            Ok(())
        }
        _ => bail!("open orders are not queried"),
    }
}
