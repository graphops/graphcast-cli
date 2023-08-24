use once_cell::sync::OnceCell;
use std::sync::Arc;

use graphcast_sdk::graphcast_agent::GraphcastAgent;

pub mod config;
pub mod messages;
pub mod operator;

/// A global static (singleton) instance of GraphcastAgent. It is useful to ensure that we have only one GraphcastAgent
/// per Radio instance, so that we can keep track of state and more easily test our Radio application.
pub static GRAPHCAST_AGENT: OnceCell<Arc<GraphcastAgent>> = OnceCell::new();
