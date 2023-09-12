use anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:3000")?;

    hc.do_get("/hello/Vlad").await?.print().await?;

    let rq_login = hc.do_post(
        "/api/login",
        json!(
            {
                "username": "demo",
                "password": "test",
            }
        ),
    );
    rq_login.await?.print().await?;

    let rq_create_ticket = hc.do_post(
        "/api/tickets",
        json!(
            {
                "name": "Ticket TEST"
            }
        ),
    );
    rq_create_ticket.await?.print().await?;

    hc.do_get("/api/tickets").await?.print().await?;

    Ok(())
}
