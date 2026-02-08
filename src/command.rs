use crate::core::Core;

pub trait Command: Send {
   fn execute(&self, core: &mut Core) -> color_eyre::Result<()>;
}
