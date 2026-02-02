use crate::{command::Command, core::Core};

pub mod arrow;
pub mod bootstrap;
pub mod update_time;

/// Actor trait for components that can execute commands
pub trait Actor {
   /// The command name this actor handles
   const NAME: &'static str;

   type Options: for<'a> TryFrom<&'a Command, Error = color_eyre::Report>;

   /// Execute the actor with the given core state and options
   fn act(core: &mut Core, options: Self::Options) -> color_eyre::Result<()>;
}
