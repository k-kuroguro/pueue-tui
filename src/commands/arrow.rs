use crate::{
   call,
   command::Command,
   commands::FetchLog,
   core::{Core, Layer},
   event::Event,
};

enum Direction {
   Up,
   Down,
}

pub struct Arrow {
   direction: Direction,
}

impl Arrow {
   pub fn up() -> Self {
      Self {
         direction: Direction::Up,
      }
   }

   pub fn down() -> Self {
      Self {
         direction: Direction::Down,
      }
   }

   fn scroll_list(core: &mut Core, direction: &Direction) {
      match direction {
         Direction::Up => {
            let i = match core.list_state.0.selected() {
               Some(i) => {
                  if i == 0 {
                     core.pueue_tasks.len() - 1
                  } else {
                     i - 1
                  }
               }
               None => 0,
            };
            core.list_state.0.select(Some(i));
            core.list_state.1 = core.list_state.1.position(i * 1);
         }
         Direction::Down => {
            let i = match core.list_state.0.selected() {
               Some(i) => {
                  if i >= core.pueue_tasks.len() - 1 {
                     0
                  } else {
                     i + 1
                  }
               }
               None => 0,
            };
            core.list_state.0.select(Some(i));
            core.list_state.1 = core.list_state.1.position(i * 1);
         }
      }
   }
}

impl Command for Arrow {
   fn execute(&self, core: &mut Core) -> color_eyre::Result<()> {
      match core.layer() {
         Layer::List => {
            Self::scroll_list(core, &self.direction);
            call!(FetchLog);
         }
         _ => {
            return Ok(());
         }
      }
      Event::Render.emit();
      Ok(())
   }
}
