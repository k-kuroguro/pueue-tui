use pueue_lib::{Task, TaskResult, TaskStatus};
use ratatui::{
   layout::Constraint,
   style::{Style, Stylize},
   widgets::{Cell, Row, StatefulWidget, Table, TableState},
};

#[derive(Clone)]
enum HeaderCell {
   Id,
   Status,
   Priority,
   EnqueueAt,
   Dependencies,
   Label,
   Command,
   Path,
   Start,
   End,
}

impl HeaderCell {
   pub const fn as_str(&self) -> &str {
      match self {
         HeaderCell::Id => "Id",
         HeaderCell::Status => "Status",
         HeaderCell::Priority => "Prio",
         HeaderCell::EnqueueAt => "Enqueue At",
         HeaderCell::Dependencies => "Deps",
         HeaderCell::Label => "Label",
         HeaderCell::Command => "Command",
         HeaderCell::Path => "Path",
         HeaderCell::Start => "Start",
         HeaderCell::End => "End",
      }
   }
}

impl<'a> From<HeaderCell> for Cell<'a> {
   fn from(value: HeaderCell) -> Self {
      Cell::new(value.as_str().to_string()).style(Style::new().bold())
   }
}

pub struct TaskTable {
   tasks: Vec<Task>,
}

impl TaskTable {
   pub fn new(tasks: &[Task]) -> Self {
      Self {
         tasks: tasks.to_vec(),
      }
   }

   fn task_to_row<'a>(task: &Task, header: &[HeaderCell]) -> Row<'a> {
      let cells: Vec<Cell> = header
         .iter()
         .map(|header_cell| Self::task_to_cell(task, header_cell))
         .collect();

      Row::new(cells)
   }

   fn task_to_cell<'a>(task: &Task, header_cell: &HeaderCell) -> Cell<'a> {
      match header_cell {
         HeaderCell::Id => Cell::new(task.id.to_string()).style(Style::new()),
         HeaderCell::Status => {
            let content = match &task.status {
               TaskStatus::Done { result, .. } => match result {
                  TaskResult::Success => "Success".to_string(),
                  TaskResult::Failed(code) => format!("Failed ({})", code),
                  TaskResult::FailedToSpawn(_) => "Failed to spawn".to_string(),
                  TaskResult::Killed => "Killed".to_string(),
                  TaskResult::Errored => "Errored".to_string(),
                  TaskResult::DependencyFailed => "Dependency failed".to_string(),
               },
               _ => task.status.to_string(),
            };
            let style = match &task.status {
               TaskStatus::Locked { .. } => Style::new(),
               TaskStatus::Stashed { .. } => Style::new().yellow(),
               TaskStatus::Queued { .. } => Style::new().yellow(),
               TaskStatus::Running { .. } => Style::new().green(),
               TaskStatus::Paused { .. } => Style::new(),
               TaskStatus::Done { result, .. } => match result {
                  TaskResult::Success => Style::new().green(),
                  _ => Style::new().red(),
               },
            };
            Cell::new(content).style(style.bold())
         }
         HeaderCell::Priority => Cell::new(task.priority.to_string()).style(Style::new()),
         HeaderCell::EnqueueAt => {
            let content = if let TaskStatus::Stashed {
               enqueue_at: Some(time),
            } = &task.status
            {
               time.format("%Y-%m-%d %H:%M:%S").to_string()
            } else {
               String::new()
            };
            Cell::new(content).style(Style::new())
         }
         HeaderCell::Dependencies => {
            let content = if task.dependencies.is_empty() {
               String::new()
            } else {
               task
                  .dependencies
                  .iter()
                  .map(|id| id.to_string())
                  .collect::<Vec<_>>()
                  .join(", ")
            };
            Cell::new(content).style(Style::new())
         }
         HeaderCell::Label => Cell::new(task.label.clone().unwrap_or_default()).style(Style::new()),
         HeaderCell::Command => Cell::new(task.command.clone()).style(Style::new()),
         HeaderCell::Path => Cell::new(task.path.to_string_lossy().to_string()).style(Style::new()),
         HeaderCell::Start => {
            let content = if let (Some(start), _) = task.start_and_end() {
               start.format("%Y-%m-%d %H:%M:%S").to_string()
            } else {
               String::new()
            };
            Cell::new(content).style(Style::new())
         }
         HeaderCell::End => {
            let content = if let (_, Some(end)) = task.start_and_end() {
               end.format("%Y-%m-%d %H:%M:%S").to_string()
            } else {
               String::new()
            };
            Cell::new(content).style(Style::new())
         }
      }
   }

   fn tasks_to_header(tasks: &[Task]) -> Vec<HeaderCell> {
      let (has_prio, has_enqueue_at, has_deps, has_label) =
         tasks.iter().fold((false, false, false, false), |acc, t| {
            (
               acc.0 || t.priority != 0,
               acc.1
                  || matches!(
                     t.status,
                     TaskStatus::Stashed {
                        enqueue_at: Some(_)
                     }
                  ),
               acc.2 || !t.dependencies.is_empty(),
               acc.3 || t.label.is_some(),
            )
         });

      [
         Some(HeaderCell::Id),
         Some(HeaderCell::Status),
         has_prio.then(|| HeaderCell::Priority),
         has_enqueue_at.then(|| HeaderCell::EnqueueAt),
         has_deps.then(|| HeaderCell::Dependencies),
         has_label.then(|| HeaderCell::Label),
         Some(HeaderCell::Command),
         Some(HeaderCell::Path),
         Some(HeaderCell::Start),
         Some(HeaderCell::End),
      ]
      .into_iter()
      .flatten()
      .collect()
   }
}

impl StatefulWidget for TaskTable {
   type State = TableState;

   fn render(
      self,
      area: ratatui::layout::Rect,
      buf: &mut ratatui::buffer::Buffer,
      state: &mut Self::State,
   ) {
      let header = Self::tasks_to_header(&self.tasks);

      let percent = 100.0 / (header.len() as f64 + 2.0);
      let mut widths = vec![];
      for col in &header {
         if matches!(col, HeaderCell::Command | HeaderCell::Path) {
            widths.push(Constraint::Percentage((percent * 2.0) as u16));
         } else {
            widths.push(Constraint::Percentage(percent as u16));
         }
      }

      let rows: Vec<Row> = self
         .tasks
         .iter()
         .map(|task| Self::task_to_row(task, &header))
         .collect();
      let table = Table::new(rows, widths)
         .header(Row::new(
            header
               .iter()
               .cloned()
               .map(|hc| Cell::from(hc))
               .collect::<Vec<Cell>>(),
         ))
         .row_highlight_style(Style::new().on_black());

      table.render(area, buf, state);
   }
}
