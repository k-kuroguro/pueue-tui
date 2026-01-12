use std::sync::Arc;

use pueue_lib::State;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
   Tick,
   Render,
   Resize(u16, u16),
   Quit,
   Error(String),

   UpdateStatus(State),
   UpdateLog(usize, Arc<[u8]>),

   RequestLog(usize),
}
