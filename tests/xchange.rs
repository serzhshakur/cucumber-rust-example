use crate::steps::MyWorld;
use anyhow::Context;
use cucumber::{writer, WorldInit};
use std::{env, fs};
use xchange_cucumber_rust::config::Profile;
mod steps;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let features_dir = "tests/features/";

    if let Profile::Dev = Profile::init() {
        dotenv::dotenv().expect("unable to setup env vars");
    }

    match env::var("REPORTS_DIR") {
        Ok(dir) => {
            let path = format!("{}/report.xml", dir);
            fs::create_dir_all(&dir).context(format!("can't create path {}", dir))?;
            let file = fs::File::create(&path).context(format!("can't create file {}", path))?;

            MyWorld::cucumber()
                .with_writer(writer::JUnit::new(file, 0))
                .run(features_dir)
                .await;
        }
        _ => {
            MyWorld::run(features_dir).await;
        }
    }
    Ok(())
}
