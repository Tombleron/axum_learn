use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};

use crate::{
    ctx::Ctx,
    error::{Error, Result},
};

#[derive(Clone, Debug, Serialize)]
pub struct Ticket {
    pub id: u64,
    pub cid: u64,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct TicketCreate {
    pub name: String,
}

#[derive(Clone)]
pub struct ModelControler {
    // TODO: mock sqlx conn
    tickets_store: Arc<Mutex<Vec<Option<Ticket>>>>,
}

impl ModelControler {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            tickets_store: Arc::default(),
        })
    }
}

impl ModelControler {
    pub async fn create_ticket(&self, ctx: Ctx, ticket_fc: TicketCreate) -> Result<Ticket> {
        let mut store = self.tickets_store.lock().unwrap();

        let id = store.len() as u64;

        let ticket = Ticket {
            id,
            cid: ctx.user_id(),
            name: ticket_fc.name,
        };

        store.push(Some(ticket.clone()));

        Ok(ticket)
    }

    pub async fn list_tickets(&self, ctx: Ctx) -> Result<Vec<Ticket>> {
        let store = self.tickets_store.lock().unwrap();

        let tickets = store.iter().filter_map(|tick| tick.clone()).collect();

        Ok(tickets)
    }

    pub async fn delete_ticket(&self, ctx: Ctx, id: u64) -> Result<Ticket> {
        let mut store = self.tickets_store.lock().unwrap();

        let ticket = store.get_mut(id as usize).and_then(|t| t.take());

        ticket.ok_or(Error::TicketDeleteIdNotFound { id })
    }
}
