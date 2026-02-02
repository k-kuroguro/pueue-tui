use std::{
   io::{Stdout, stdout},
   ops::{Deref, DerefMut},
   time::Duration,
};

use color_eyre::eyre::eyre;
use crossterm::{
   cursor,
   event::{
      DisableBracketedPaste, DisableMouseCapture, EnableBracketedPaste, EnableMouseCapture,
      Event as CrosstermEvent, EventStream, KeyEvent, KeyEventKind, MouseEvent,
   },
   terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};
use futures::{FutureExt, StreamExt};
use ratatui::backend::CrosstermBackend;
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;

use crate::event::Event;

#[derive(Clone)]
pub struct TerminalOptions {
   pub mouse: bool,
   pub paste: bool,
}

pub struct Terminal {
   inner: ratatui::Terminal<CrosstermBackend<Stdout>>,
   cancellation_token: CancellationToken,
   task: JoinHandle<()>,
   options: TerminalOptions,
}

impl Terminal {
   pub fn new(options: TerminalOptions) -> color_eyre::Result<Self> {
      Ok(Self {
         inner: ratatui::Terminal::new(CrosstermBackend::new(stdout()))?,
         cancellation_token: CancellationToken::new(),
         task: tokio::spawn(async {}),
         options,
      })
   }

   pub fn start(&mut self) {
      self.cancel(); // Cancel any existing task.
      self.cancellation_token = CancellationToken::new();
      let event_loop = Self::event_loop(self.cancellation_token.clone());
      self.task = tokio::spawn(async {
         event_loop.await;
      });
   }

   async fn event_loop(cancellation_token: CancellationToken) {
      let mut event_stream = EventStream::new();
      loop {
         let event = tokio::select! {
            _ = cancellation_token.cancelled() => {
               break;
            }
            crossterm_event = event_stream.next().fuse() => match crossterm_event {
               Some(Ok(event)) => match event {
                  CrosstermEvent::Key(key) => Event::Key(key),
                  CrosstermEvent::Mouse(mouse) => Event::Mouse(mouse),
                  CrosstermEvent::Resize(..) => Event::Resize,
                  CrosstermEvent::FocusGained => Event::Focus,
                  CrosstermEvent::Paste(s) => Event::Paste(s),
                  _ => continue,
               }
               Some(Err(e)) => Event::Error(eyre!(e)),
               None => break, // the event stream has stopped and will not produce any more events
             },
         };
         event.emit();
      }
      cancellation_token.cancel();
   }

   pub fn stop(&self) -> color_eyre::Result<()> {
      self.cancel();
      let mut counter = 0;
      while !self.task.is_finished() {
         std::thread::sleep(Duration::from_millis(1));
         counter += 1;
         if counter > 50 {
            self.task.abort();
         }
         if counter > 100 {
            break;
         }
      }
      Ok(())
   }

   pub fn enter(&mut self) -> color_eyre::Result<()> {
      crossterm::terminal::enable_raw_mode()?;
      crossterm::execute!(stdout(), EnterAlternateScreen, cursor::Hide)?;
      if self.options.mouse {
         crossterm::execute!(stdout(), EnableMouseCapture)?;
      }
      if self.options.paste {
         crossterm::execute!(stdout(), EnableBracketedPaste)?;
      }
      self.start();
      Ok(())
   }

   pub fn exit(&mut self) -> color_eyre::Result<()> {
      self.stop()?;
      if crossterm::terminal::is_raw_mode_enabled()? {
         self.flush()?;
         if self.options.paste {
            crossterm::execute!(stdout(), DisableBracketedPaste)?;
         }
         if self.options.mouse {
            crossterm::execute!(stdout(), DisableMouseCapture)?;
         }
         crossterm::execute!(stdout(), LeaveAlternateScreen, cursor::Show)?;
         crossterm::terminal::disable_raw_mode()?;
      }
      Ok(())
   }

   pub fn cancel(&self) {
      self.cancellation_token.cancel();
   }
}

impl Drop for Terminal {
   fn drop(&mut self) {
      self.exit().unwrap();
   }
}

impl Deref for Terminal {
   type Target = ratatui::Terminal<CrosstermBackend<Stdout>>;

   fn deref(&self) -> &Self::Target {
      &self.inner
   }
}

impl DerefMut for Terminal {
   fn deref_mut(&mut self) -> &mut Self::Target {
      &mut self.inner
   }
}
