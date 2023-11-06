use anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let base_url = "http://localhost:8080";
    let hc = httpc_test::new_client(base_url)?;

    hc.do_get("/hello?name=tawan").await?.print().await?;

    let req_login = hc.do_post(
        "/api/login",
        json!({
            "username": "demo1",
            "pwd": "welcome"
        }),
    );
    req_login.await?.print().await?;

    hc.do_get("/hello2/tawan2").await?.print().await?;

    let req_create_ticket = hc.do_post(
        "/api/tickets",
        json!({
            "title": "Ticket AAA"
        }),
    );

    req_create_ticket.await?.print().await?;

    hc.do_get("/api/tickets").await?.print().await?;

    // hc.do_delete("/api/tickets/0").await?.print().await?;

    Ok(())
}