use crate::app::{App, AppResult};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        // Exit application on `ESC` or `q`
        KeyCode::Esc | KeyCode::Char('q') => {
            app.quit();
        }
        // Exit application on `Ctrl-C`
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit();
            }
        }
        KeyCode::Char('k') | KeyCode::Up => app.previous_unanswered_question(),
        KeyCode::Char('j') | KeyCode::Down => app.next_unanswered_question(),
        KeyCode::Char('r') => app.refresh_unanswered_questions(),
        KeyCode::Char('n') => app.next_question_page(),
        KeyCode::Char('p') => app.previous_question_page(),
        KeyCode::Char('o') => app.open_selected_question(),
        KeyCode::Char(' ') => app.toogle_question_body(),
        _ => {}
    }
    Ok(())
}
