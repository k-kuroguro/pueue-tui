/// Command represents a user action that can be executed.
#[derive(Debug, Clone)]
pub struct Command {
   /// The name of the command
   pub name: String,
   /// Arguments for the command
   pub args: Vec<String>,
}

impl Command {
   /// Create a new command with the given name
   pub fn new(name: impl Into<String>) -> Self {
      Self {
         name: name.into(),
         args: Vec::new(),
      }
   }

   /// Create a new command with arguments
   pub fn with_args(name: impl Into<String>, args: Vec<String>) -> Self {
      Self {
         name: name.into(),
         args,
      }
   }

   /// Add an argument to the command
   pub fn arg(mut self, arg: impl Into<String>) -> Self {
      self.args.push(arg.into());
      self
   }

   /// Get the first argument as a string, if it exists
   pub fn first_arg(&self) -> Option<&str> {
      self.args.first().map(|s| s.as_str())
   }

   /// Parse first argument as i32
   pub fn first_i32(&self) -> Option<i32> {
      self.first_arg().and_then(|s| s.parse().ok())
   }
}
