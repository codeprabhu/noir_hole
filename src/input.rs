use crossterm::event::{Event, KeyCode, MouseButton, MouseEventKind};

pub enum Action {
    Quit,
    Orbit { dx: f32, dy: f32 },
    Zoom(f32),
}

#[derive(Default)]
pub struct InputState {
    dragging: bool,
    last_mouse: Option<(u16, u16)>,
}

impl InputState {
    pub fn handle(&mut self, event: Event) -> Option<Action> {
        match event {
            Event::Key(key) => match key.code {
                KeyCode::Char('q') | KeyCode::Esc => Some(Action::Quit),
                KeyCode::Char('+') | KeyCode::Char('=') | KeyCode::Up => Some(Action::Zoom(-0.5)),
                KeyCode::Char('-') | KeyCode::Down => Some(Action::Zoom(0.5)),
                _ => None,
            },
            Event::Mouse(mouse) => match mouse.kind {
                MouseEventKind::Down(MouseButton::Left) => {
                    self.dragging = true;
                    self.last_mouse = Some((mouse.column, mouse.row));
                    None
                }
                MouseEventKind::Up(MouseButton::Left) => {
                    self.dragging = false;
                    self.last_mouse = None;
                    None
                }
                MouseEventKind::Drag(MouseButton::Left) if self.dragging => {
                    let previous = self.last_mouse.replace((mouse.column, mouse.row));
                    previous.map(|(x, y)| Action::Orbit {
                        dx: (mouse.column as f32 - x as f32) * 0.012,
                        dy: (mouse.row as f32 - y as f32) * 0.012,
                    })
                }
                MouseEventKind::ScrollUp => Some(Action::Zoom(-0.5)),
                MouseEventKind::ScrollDown => Some(Action::Zoom(0.5)),
                _ => None,
            },
            _ => None,
        }
    }
}
