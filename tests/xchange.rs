use crate::steps::MyWorld;
use cucumber::{writer, WorldInit};
use std::fs;
mod steps;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let reports_dir = std::env::var("REPORTS_DIR").unwrap_or_else(|_| "./reports".to_string());
    let file = fs::File::create(dbg!(format!("{}/report.xml", reports_dir)))?;

    MyWorld::cucumber()
        .with_writer(writer::JUnit::new(file, 0))
        .run("tests/features/")
        .await;
    Ok(())
}
