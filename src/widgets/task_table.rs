use pueue_lib::{Task, TaskResult, TaskStatus};
use ratatui::{
   buffer::Buffer,
   layout::{Constraint, Layout, Rect},
   style::Style,
   widgets::{
      Cell, Row, Scrollbar, ScrollbarOrientation, ScrollbarState, StatefulWidget, Table, TableState,
   },
};

//TODO: truncate long command and path strings with "..."

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

const TIME_FORMAT: &str = "%Y-%m-%d %H:%M:%S";

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
            let content = Self::task_status_to_string(&task.status);
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
               time.format(TIME_FORMAT).to_string()
            } else {
               String::new()
            };
            Cell::new(content).style(Style::new())
         }
         HeaderCell::Dependencies => {
            let content = Self::dependencies_to_string(&task.dependencies);
            Cell::new(content).style(Style::new())
         }
         HeaderCell::Label => Cell::new(task.label.clone().unwrap_or_default()).style(Style::new()),
         HeaderCell::Command => Cell::new(task.command.clone()).style(Style::new()),
         HeaderCell::Path => Cell::new(task.path.to_string_lossy().to_string()).style(Style::new()),
         HeaderCell::Start => {
            let content = if let (Some(start), _) = task.start_and_end() {
               start.format(TIME_FORMAT).to_string()
            } else {
               String::new()
            };
            Cell::new(content).style(Style::new())
         }
         HeaderCell::End => {
            let content = if let (_, Some(end)) = task.start_and_end() {
               end.format(TIME_FORMAT).to_string()
            } else {
               String::new()
            };
            Cell::new(content).style(Style::new())
         }
      }
   }

   fn task_status_to_string(status: &TaskStatus) -> String {
      match &status {
         TaskStatus::Done { result, .. } => match result {
            TaskResult::Success => "Success".to_string(),
            TaskResult::Failed(code) => format!("Failed ({})", code),
            TaskResult::FailedToSpawn(_) => "Failed to spawn".to_string(),
            TaskResult::Killed => "Killed".to_string(),
            TaskResult::Errored => "Errored".to_string(),
            TaskResult::DependencyFailed => "Dependency failed".to_string(),
         },
         _ => status.to_string(),
      }
   }

   fn dependencies_to_string(dependencies: &[usize]) -> String {
      dependencies
         .iter()
         .map(|id| id.to_string())
         .collect::<Vec<_>>()
         .join(", ")
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

   fn calc_widths(header: &[HeaderCell], tasks: &[Task]) -> Vec<Constraint> {
      let (
         max_id_width,
         max_status_width,
         max_priority_width,
         max_dependencies_width,
         max_label_width,
      ) = tasks.iter().fold(
         (
            HeaderCell::Id.as_str().len(),
            HeaderCell::Status.as_str().len(),
            HeaderCell::Priority.as_str().len(),
            HeaderCell::Dependencies.as_str().len(),
            HeaderCell::Label.as_str().len(),
         ),
         |(id, status, prio, deps, label), t| {
            (
               id.max(t.id.to_string().len()),
               status.max(Self::task_status_to_string(&t.status).len()),
               prio.max(t.priority.to_string().len()),
               deps.max(Self::dependencies_to_string(&t.dependencies).len()),
               label.max(t.label.as_ref().map_or(0, |l| l.len())),
            )
         },
      );

      header
         .iter()
         .map(|col| match col {
            HeaderCell::Id => Constraint::Max(max_id_width as u16),
            HeaderCell::Status => Constraint::Max(max_status_width as u16),
            HeaderCell::Priority => Constraint::Max(max_priority_width as u16),
            HeaderCell::EnqueueAt | HeaderCell::Start | HeaderCell::End => {
               Constraint::Max("YYYY-MM-DD HH:MM:SS".len() as u16)
            }
            HeaderCell::Dependencies => Constraint::Max(max_dependencies_width as u16),
            HeaderCell::Label => Constraint::Max(max_label_width as u16),
            HeaderCell::Command | HeaderCell::Path => {
               Constraint::Min(8 /* no real reason, adjust later */)
            }
         })
         .collect()
   }
}

pub type TaskTableState = (TableState, ScrollbarState);

impl StatefulWidget for TaskTable {
   type State = TaskTableState;

   fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
      (*state).1 = state.1.content_length(self.tasks.len());

      let header = Self::tasks_to_header(&self.tasks);

      let widths = Self::calc_widths(&header, &self.tasks);
      let rows: Vec<Row> = self
         .tasks
         .iter()
         .map(|task| Self::task_to_row(task, &header))
         .collect();

      let is_needed_scrollbar = area.height.saturating_sub(1) < self.tasks.len() as u16;
      let (table_area, scroll_bar_area) = (|| {
         if is_needed_scrollbar {
            let [table_area, scroll_bar_area] =
               Layout::horizontal([Constraint::Fill(1), Constraint::Length(2)]).areas(area);
            let scroll_bar_area = Rect {
               x: scroll_bar_area.x,
               y: table_area.y.saturating_add(1),
               width: scroll_bar_area.width,
               height: table_area.height.saturating_sub(1),
            }; // Adjust for table header

            (table_area, Some(scroll_bar_area))
         } else {
            (area, None)
         }
      })();

      let table = Table::new(rows, widths)
         .header(Row::new(
            header
               .iter()
               .cloned()
               .map(|hc| Cell::from(hc))
               .collect::<Vec<Cell>>(),
         ))
         .column_spacing(2)
         .row_highlight_style(Style::new().on_black());
      table.render(table_area, buf, &mut state.0);

      if let Some(scroll_bar_area) = scroll_bar_area {
         let scroll_bar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .begin_symbol(None)
            .end_symbol(None);
         scroll_bar.render(scroll_bar_area, buf, &mut state.1);
      }
   }
}
