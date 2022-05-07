use std::convert::Infallible;

use anyhow::bail;
use async_trait::async_trait;
use chrono::{DateTime, NaiveDateTime, Utc};
use cucumber::{then, when, World, WorldInit};
use xchange_cucumber_rust::{
    api::api::{MarketApi, UserApi},
    responses::{OpenOrderResponse, ServerTimeResponse},
    Deps,
};

#[derive(Debug)]
enum State {
    Empty,
    ServerTime(ServerTimeResponse),
    OpenOrders(OpenOrderResponse),
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
            let now = Utc::now().naive_utc();
            let from_unix = NaiveDateTime::from_timestamp(data.unixtime, 0);
            let from_rfc_string = DateTime::parse_from_rfc2822(&data.rfc1123)
                .unwrap()
                .naive_utc();

            assert!(now.signed_duration_since(from_unix).num_seconds() < 2);
            assert!(now.signed_duration_since(from_rfc_string).num_seconds() < 2);
        }
        _ => panic!("server time is not retrieved"),
    }
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
