use std::convert::Infallible;

use async_trait::async_trait;
use chrono::{DateTime, NaiveDateTime, Utc};
use cucumber::{then, when, World, WorldInit};
use xchange_cucumber_rust::{api::api::MarketApi, responses::ServerTimeResponse, Deps};

#[derive(Debug)]
enum State {
    Empty,
    ServerTime(ServerTimeResponse),
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
async fn retrieve_server_time(world: &mut MyWorld) {
    let res = world.deps.api.get_server_time().await.unwrap();
    world.data = State::ServerTime(res.unwrap());
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

#[tokio::main]
async fn main() {
    MyWorld::cucumber().run_and_exit("tests/features/").await;
    // AnimalWorld::run("tests/features/").await;
}
