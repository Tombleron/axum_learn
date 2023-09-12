use axum::{
    extract::{Path, State},
    routing::{delete, post},
    Json, Router,
};

use crate::{
    ctx::Ctx,
    error::Result,
    model::{ModelControler, Ticket, TicketCreate},
};

pub fn routes(mc: ModelControler) -> Router {
    Router::new()
        .route("/tickets", post(create_ticket).get(list_tickets))
        .route("/tickets/:id", delete(delete_ticket))
        .with_state(mc)
}

async fn create_ticket(
    State(mc): State<ModelControler>,
    ctx: Ctx,
    Json(ticket_fc): Json<TicketCreate>,
) -> Result<Json<Ticket>> {
    println!("->> {:<12} - create_ticket", "HANDLER");

    let ticket = mc.create_ticket(ctx, ticket_fc).await?;

    Ok(Json(ticket))
}

async fn list_tickets(State(mc): State<ModelControler>, ctx: Ctx) -> Result<Json<Vec<Ticket>>> {
    println!("->> {:<12} - list_tickets", "HANDLER");

    let ticket = mc.list_tickets(ctx).await?;

    Ok(Json(ticket))
}

async fn delete_ticket(
    State(mc): State<ModelControler>,
    ctx: Ctx,
    Path(ticket_id): Path<u64>,
) -> Result<Json<Ticket>> {
    println!("->> {:<12} - delete_ticket", "HANDLER");

    let ticket = mc.delete_ticket(ctx, ticket_id).await?;

    Ok(Json(ticket))
}
