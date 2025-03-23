#![allow(unused)]

use anyhow::Result;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:4000")?;

    hc.do_get("/lorem").await?.print().await?;

    Ok(())
} // end func
