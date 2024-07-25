use anyhow::{Ok, Result};
use sol_dash::run;

#[tokio::main]
async fn main() -> Result<()> {
    run().await?;
    Ok(())
}
