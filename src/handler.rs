use crate::app::{App, AppResult, CurrentApp};
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

        _ => match app.current_app {
            CurrentApp::UnansweredQuestionsView => {
                handle_key_events_unanswered(key_event, app)?;
            }
            CurrentApp::QuestionDetailView => {
                handle_key_events_question_reader(key_event, app)?;
            }
        },
    }
    Ok(())
}

fn handle_key_events_unanswered(key_event: KeyEvent, parent: &mut App) -> AppResult<()> {
    let app = &mut parent.unanswered_questions_view;

    match key_event.code {
        KeyCode::Char('k') | KeyCode::Up => app.previous_unanswered_question(),
        KeyCode::Char('j') | KeyCode::Down => app.next_unanswered_question(),
        KeyCode::Char('r') => app.refresh_unanswered_questions(),
        KeyCode::Char('n') => app.next_question_page(),
        KeyCode::Char('p') => app.previous_question_page(),
        KeyCode::Char('o') => app.open_selected_question(),
        KeyCode::Char(' ') => {
            let question = app.get_selected_question();
            parent
                .question_reader_view
                .set_question(question, parent.current_app);
            parent.current_app = CurrentApp::QuestionDetailView;
        }
        _ => {}
    }
    Ok(())
}

fn handle_key_events_question_reader(key_event: KeyEvent, parent: &mut App) -> AppResult<()> {
    let app = &mut parent.question_reader_view;

    match key_event.code {
        KeyCode::Char('k') | KeyCode::Up => app.previous_line(),
        KeyCode::Char('j') | KeyCode::Down => app.next_line(),
        KeyCode::Char('o') => app.open_question(),
        KeyCode::Char(' ') => {
            parent.current_app = app.parent;
        }
        _ => {}
    }
    Ok(())
}
