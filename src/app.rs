use std::{
   sync::atomic::Ordering,
   time::{Duration, Instant},
};

use crate::{
   actors::bootstrap::Bootstrap,
   cli::CliArgs,
   core::Core,
   dispatcher::Dispatcher,
   event::{Event, NEED_RENDER},
   terminal::{Terminal, TerminalOptions},
   ui,
};

const TERMINAL_OPTIONS: TerminalOptions = TerminalOptions {
   mouse: true,
   paste: true,
};

pub struct App {
   pub(crate) core: Core,
}

impl App {
   const MAX_EVENTS: usize = 50;
   const FRAME_TIME: Duration = Duration::from_millis(1000 / 60); // 60 FPS

   pub async fn new(_options: &CliArgs) -> color_eyre::Result<Self> {
      Ok(Self { core: Core::new() })
   }

   pub async fn run(&mut self) -> color_eyre::Result<()> {
      Self::set_panic_hook();
      
      // Bootstrap: Initialize background tasks and state
      Bootstrap::execute(&mut self.core)?;

      let mut event_rx = Event::init()?;
      let mut terminal = Terminal::new(TERMINAL_OPTIONS)?;
      terminal.enter()?;

      let mut events = Vec::with_capacity(Self::MAX_EVENTS);
      let mut last_render = Instant::now();
      let mut timeout = None;

      self.render(&mut terminal)?;
      loop {
         if let Some(t) = timeout.take() {
            tokio::select! {
               _ = tokio::time::sleep(t) => {
                  if NEED_RENDER.load(Ordering::Relaxed) {
                     self.render(&mut terminal)?;
                     NEED_RENDER.store(false, Ordering::Relaxed);
                     last_render = Instant::now();
                  }
               }
               n = event_rx.recv_many(&mut events, Self::MAX_EVENTS) => {
                  if n == 0 { break; }
                  self.process_events(&mut events)?;
               }
            }
         } else {
            if event_rx.recv_many(&mut events, Self::MAX_EVENTS).await == 0 {
               break;
            }
            self.process_events(&mut events)?;
         }

         if NEED_RENDER.load(Ordering::Relaxed) {
            let elapsed = last_render.elapsed();

            if elapsed >= Self::FRAME_TIME {
               self.render(&mut terminal)?;
               NEED_RENDER.store(false, Ordering::Relaxed);
               last_render = Instant::now();
            } else {
               timeout = Some(Self::FRAME_TIME - elapsed);
            }
         }

         if self.core.should_quit {
            break;
         }
      }
      terminal.exit()?;

      Ok(())
   }

   fn process_events(&mut self, events: &mut Vec<Event>) -> color_eyre::Result<()> {
      let mut dispatcher = Dispatcher::new(self);
      for event in events.drain(..) {
         dispatcher.dispatch(event)?;
      }
      Ok(())
   }

   // System-level commands - called directly for performance
   
   pub(crate) fn quit(&mut self) -> color_eyre::Result<()> {
      // Shutdown background tasks before quitting
      self.core.tasks.shutdown();
      self.core.should_quit = true;
      Ok(())
   }

   pub(crate) fn resize(&mut self) -> color_eyre::Result<()> {
      NEED_RENDER.store(true, Ordering::Relaxed);
      Ok(())
   }

   pub(crate) fn focus(&mut self) -> color_eyre::Result<()> {
      // TODO: Handle focus events
      Ok(())
   }

   pub(crate) fn paste(&mut self, _content: String) -> color_eyre::Result<()> {
      // TODO: Handle paste events
      Ok(())
   }

   pub(crate) fn render_request(&mut self) -> color_eyre::Result<()> {
      NEED_RENDER.store(true, Ordering::Relaxed);
      Ok(())
   }

   fn render(&self, terminal: &mut Terminal) -> color_eyre::Result<()> {
      terminal.draw(|f| {
         ui::render(f, &self.core);
      })?;
      Ok(())
   }

   fn set_panic_hook() {
      let hook = std::panic::take_hook();
      std::panic::set_hook(Box::new(move |info| {
         if let Ok(mut t) = Terminal::new(TERMINAL_OPTIONS) {
            let _ = t.exit();
         }
         hook(info);
      }));
   }
}
