use pueue_lib::State;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Action {
   Tick,
   Render,
   Resize(u16, u16),
   Quit,
   Error(String),
   UpdateStatus(State),
}
