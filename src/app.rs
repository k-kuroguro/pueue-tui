use std::collections::HashMap;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::prelude::Rect;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;

use crate::{
   action::Action,
   cli::CliArgs,
   client::Client,
   components::{Component, home::Home},
   tui::{Event, Tui, TuiConfig},
};

pub struct App {
   status_reload_rate: f64, // Added field
   components: Vec<Box<dyn Component>>,
   should_quit: bool,
   mode: Mode,
   last_tick_key_events: Vec<KeyEvent>,
   action_tx: mpsc::UnboundedSender<Action>,
   action_rx: mpsc::UnboundedReceiver<Action>,
   keymaps: HashMap<Mode, HashMap<Vec<KeyEvent>, Action>>,
   client: Client,
   tui_config: TuiConfig,
}

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Mode {
   #[default]
   Home,
}

impl App {
   pub async fn new(opt: &CliArgs) -> color_eyre::Result<Self> {
      let (action_tx, action_rx) = mpsc::unbounded_channel();
      Ok(Self {
         status_reload_rate: 1.0,
         components: vec![Box::new(Home::new())],
         should_quit: false,
         mode: Mode::Home,
         last_tick_key_events: Vec::new(),
         action_tx,
         action_rx,
         keymaps: {
            let mut map = HashMap::new();

            let mut home = HashMap::new();
            home.insert(parse_key_sequence("<q>").unwrap(), Action::Quit);
            home.insert(parse_key_sequence("<Ctrl-d>").unwrap(), Action::Quit);

            map.insert(Mode::Home, home);
            map
         },
         client: Client::new(&opt.config, &opt.profile).await?,
         tui_config: TuiConfig {
            frame_rate: 60.0,
            tick_rate: 4.0,
            mouse: true,
            paste: false,
         },
      })
   }

   pub async fn run(&mut self) -> color_eyre::Result<()> {
      self.set_panic_hook();

      let mut tui = Tui::try_from(&self.tui_config)?;
      tui.enter()?;

      let status_action_tx = self.action_tx.clone();
      let status_client = self.client.clone();
      let status_reload_duration = std::time::Duration::from_secs_f64(self.status_reload_rate);
      tokio::spawn(async move {
         loop {
            match status_client.status().await {
               Ok(state) => {
                  let _ = status_action_tx.send(Action::UpdateStatus(state));
               }
               Err(e) => {
                  let _ = status_action_tx
                     .send(Action::Error(format!("Failed to fetch status: {:?}", e)));
               }
            }
            tokio::time::sleep(status_reload_duration).await;
         }
      });

      for component in self.components.iter_mut() {
         component.register_action_handler(self.action_tx.clone())?;
      }
      for component in self.components.iter_mut() {
         component.init(tui.size()?)?;
      }

      loop {
         self.handle_events(&mut tui).await?;
         self.handle_actions(&mut tui)?;
         if self.should_quit {
            tui.stop()?;
            break;
         }
      }
      tui.exit()?;
      Ok(())
   }

   async fn handle_events(&mut self, tui: &mut Tui) -> color_eyre::Result<()> {
      let Some(event) = tui.next_event().await else {
         return Ok(());
      };
      let action_tx = self.action_tx.clone();
      match event {
         Event::Quit => action_tx.send(Action::Quit)?,
         Event::Tick => action_tx.send(Action::Tick)?,
         Event::Render => action_tx.send(Action::Render)?,
         Event::Resize(x, y) => action_tx.send(Action::Resize(x, y))?,
         Event::Key(key) => self.handle_key_event(key)?,
         _ => {}
      }
      for component in self.components.iter_mut() {
         if let Some(action) = component.handle_events(Some(event.clone()))? {
            action_tx.send(action)?;
         }
      }
      Ok(())
   }

   fn handle_key_event(&mut self, key: KeyEvent) -> color_eyre::Result<()> {
      let action_tx = self.action_tx.clone();
      let Some(keymap) = self.keymaps.get(&self.mode) else {
         return Ok(());
      };
      match keymap.get(&vec![key]) {
         Some(action) => {
            action_tx.send(action.clone())?;
         }
         _ => {
            // If the key was not handled as a single key action,
            // then consider it for multi-key combinations.
            self.last_tick_key_events.push(key);

            // Check for multi-key combinations
            if let Some(action) = keymap.get(&self.last_tick_key_events) {
               action_tx.send(action.clone())?;
            }
         }
      }
      Ok(())
   }

   fn handle_actions(&mut self, tui: &mut Tui) -> color_eyre::Result<()> {
      while let Ok(action) = self.action_rx.try_recv() {
         match action {
            Action::Tick => {
               self.last_tick_key_events.drain(..);
            }
            Action::Quit => self.should_quit = true,
            Action::Resize(w, h) => self.handle_resize(tui, w, h)?,
            Action::Render => self.render(tui)?,
            _ => {}
         }
         for component in self.components.iter_mut() {
            if let Some(action) = component.update(action.clone())? {
               self.action_tx.send(action)?
            };
         }
      }
      Ok(())
   }

   fn handle_resize(&mut self, tui: &mut Tui, w: u16, h: u16) -> color_eyre::Result<()> {
      tui.resize(Rect::new(0, 0, w, h))?;
      self.render(tui)?;
      Ok(())
   }

   fn render(&mut self, tui: &mut Tui) -> color_eyre::Result<()> {
      tui.draw(|frame| {
         for component in self.components.iter_mut() {
            if let Err(err) = component.draw(frame, frame.area()) {
               let _ = self
                  .action_tx
                  .send(Action::Error(format!("Failed to draw: {:?}", err)));
            }
         }
      })?;
      Ok(())
   }

   fn set_panic_hook(&self) {
      let hook = std::panic::take_hook();
      let tui_config = self.tui_config.clone();
      std::panic::set_hook(Box::new(move |info| {
         if let Ok(mut t) = Tui::try_from(&tui_config) {
            let _ = t.exit();
         }
         hook(info);
      }));
   }
}

fn parse_key_event(raw: &str) -> color_eyre::Result<KeyEvent, String> {
   let raw_lower = raw.to_ascii_lowercase();
   let (remaining, modifiers) = extract_modifiers(&raw_lower);
   parse_key_code_with_modifiers(remaining, modifiers)
}

fn extract_modifiers(raw: &str) -> (&str, KeyModifiers) {
   let mut modifiers = KeyModifiers::empty();
   let mut current = raw;

   loop {
      match current {
         rest if rest.starts_with("ctrl-") => {
            modifiers.insert(KeyModifiers::CONTROL);
            current = &rest[5..];
         }
         rest if rest.starts_with("alt-") => {
            modifiers.insert(KeyModifiers::ALT);
            current = &rest[4..];
         }
         rest if rest.starts_with("shift-") => {
            modifiers.insert(KeyModifiers::SHIFT);
            current = &rest[6..];
         }
         _ => break, // break out of the loop if no known prefix is detected
      };
   }

   (current, modifiers)
}

fn parse_key_code_with_modifiers(
   raw: &str,
   mut modifiers: KeyModifiers,
) -> color_eyre::Result<KeyEvent, String> {
   let c = match raw {
      "esc" => KeyCode::Esc,
      "enter" => KeyCode::Enter,
      "left" => KeyCode::Left,
      "right" => KeyCode::Right,
      "up" => KeyCode::Up,
      "down" => KeyCode::Down,
      "home" => KeyCode::Home,
      "end" => KeyCode::End,
      "pageup" => KeyCode::PageUp,
      "pagedown" => KeyCode::PageDown,
      "backtab" => {
         modifiers.insert(KeyModifiers::SHIFT);
         KeyCode::BackTab
      }
      "backspace" => KeyCode::Backspace,
      "delete" => KeyCode::Delete,
      "insert" => KeyCode::Insert,
      "f1" => KeyCode::F(1),
      "f2" => KeyCode::F(2),
      "f3" => KeyCode::F(3),
      "f4" => KeyCode::F(4),
      "f5" => KeyCode::F(5),
      "f6" => KeyCode::F(6),
      "f7" => KeyCode::F(7),
      "f8" => KeyCode::F(8),
      "f9" => KeyCode::F(9),
      "f10" => KeyCode::F(10),
      "f11" => KeyCode::F(11),
      "f12" => KeyCode::F(12),
      "space" => KeyCode::Char(' '),
      "hyphen" => KeyCode::Char('-'),
      "minus" => KeyCode::Char('-'),
      "tab" => KeyCode::Tab,
      c if c.len() == 1 => {
         let mut c = c.chars().next().unwrap();
         if modifiers.contains(KeyModifiers::SHIFT) {
            c = c.to_ascii_uppercase();
         }
         KeyCode::Char(c)
      }
      _ => return Err(format!("Unable to parse {raw}")),
   };
   Ok(KeyEvent::new(c, modifiers))
}

fn key_event_to_string(key_event: &KeyEvent) -> String {
   let char;
   let key_code = match key_event.code {
      KeyCode::Backspace => "backspace",
      KeyCode::Enter => "enter",
      KeyCode::Left => "left",
      KeyCode::Right => "right",
      KeyCode::Up => "up",
      KeyCode::Down => "down",
      KeyCode::Home => "home",
      KeyCode::End => "end",
      KeyCode::PageUp => "pageup",
      KeyCode::PageDown => "pagedown",
      KeyCode::Tab => "tab",
      KeyCode::BackTab => "backtab",
      KeyCode::Delete => "delete",
      KeyCode::Insert => "insert",
      KeyCode::F(c) => {
         char = format!("f({c})");
         &char
      }
      KeyCode::Char(' ') => "space",
      KeyCode::Char(c) => {
         char = c.to_string();
         &char
      }
      KeyCode::Esc => "esc",
      KeyCode::Null => "",
      KeyCode::CapsLock => "",
      KeyCode::Menu => "",
      KeyCode::ScrollLock => "",
      KeyCode::Media(_) => "",
      KeyCode::NumLock => "",
      KeyCode::PrintScreen => "",
      KeyCode::Pause => "",
      KeyCode::KeypadBegin => "",
      KeyCode::Modifier(_) => "",
   };

   let mut modifiers = Vec::with_capacity(3);

   if key_event.modifiers.intersects(KeyModifiers::CONTROL) {
      modifiers.push("ctrl");
   }

   if key_event.modifiers.intersects(KeyModifiers::SHIFT) {
      modifiers.push("shift");
   }

   if key_event.modifiers.intersects(KeyModifiers::ALT) {
      modifiers.push("alt");
   }

   let mut key = modifiers.join("-");

   if !key.is_empty() {
      key.push('-');
   }
   key.push_str(key_code);

   key
}

fn parse_key_sequence(raw: &str) -> color_eyre::Result<Vec<KeyEvent>, String> {
   if raw.chars().filter(|c| *c == '>').count() != raw.chars().filter(|c| *c == '<').count() {
      return Err(format!("Unable to parse `{}`", raw));
   }
   let raw = if !raw.contains("><") {
      let raw = raw.strip_prefix('<').unwrap_or(raw);
      let raw = raw.strip_prefix('>').unwrap_or(raw);
      raw
   } else {
      raw
   };
   let sequences = raw
      .split("><")
      .map(|seq| {
         if let Some(s) = seq.strip_prefix('<') {
            s
         } else if let Some(s) = seq.strip_suffix('>') {
            s
         } else {
            seq
         }
      })
      .collect::<Vec<_>>();

   sequences.into_iter().map(parse_key_event).collect()
}
