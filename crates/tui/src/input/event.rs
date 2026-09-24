use crossterm::event::{Event, KeyCode, KeyModifiers};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UiEvent<'paste> {
    Paste(&'paste str),
    Press(UiPress),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UiPress {
    Right,
    Left,
    Up,
    Down,
    Enter,
    Escape,
    Tab,
    Char(char),
}

impl<'a> UiEvent<'a> {
    pub fn parse_event(e: &'a Event) -> Option<UiEvent<'a>> {
        Some(match e {
            Event::Paste(paste) => UiEvent::Paste(paste),
            Event::Key(key) if key.is_press() => match key.code {
                KeyCode::Char('v' | 'V') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    // handle paste
                    todo!()
                }
                code => UiEvent::Press(match code {
                    KeyCode::Right => UiPress::Right,
                    KeyCode::Left => UiPress::Left,
                    KeyCode::Up => UiPress::Up,
                    KeyCode::Down => UiPress::Down,
                    KeyCode::Enter => UiPress::Enter,
                    KeyCode::Esc => UiPress::Escape,
                    KeyCode::Tab => UiPress::Tab,
                    KeyCode::Char(c) => UiPress::Char(c),
                    _ => return None,
                }),
            },
            _ => return None,
        })
    }
}
