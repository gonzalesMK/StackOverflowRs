use ratatui::{
    backend::Backend,
    layout::Alignment,
    prelude::{Constraint, Layout},
    style::{Color, Modifier, Style},
    symbols::scrollbar,
    text::{Line, Span},
    widgets::{
        Block, BorderType, Borders, List, ListItem, Paragraph, Scrollbar, ScrollbarOrientation,
        Wrap,
    },
    Frame,
};

use crate::app::{App, CurrentApp, Question, QuestionReaderView, UnansweredQuestionsView};

fn render_question(question: &Question, size: usize) -> ListItem {
    // format  title, description and link
    let mut content = vec![Line::from(Span::styled(
        format!("{}) {}", size, question.title),
        Style::default().add_modifier(Modifier::BOLD),
    ))];

    if question.show_body {
        content.extend(
            question
                .body
                .split("\n")
                .map(|i| Line::from(Span::raw(i)))
                .collect::<Vec<Line>>(),
        );
    } else {
        content.push(Line::from(Span::raw(question.description.as_str())));
    }

    content.push(Line::from(Span::raw(format!(
        "Tags: {}\t Answers: {}\n",
        question.tags.join(", "),
        question.answer_count
    ))));
    content.push(Line::from(Span::raw("")));

    ListItem::new(content)
}
/// Renders the user interface widgets.
pub fn render<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
    match app.current_app {
        CurrentApp::QuestionDetailView => {
            render_question_detail_view(&mut app.question_reader_view, frame)
        }
        CurrentApp::UnansweredQuestionsView => {
            render_unanswered_questions(&mut app.unanswered_questions_view, frame)
        }
    }
}

pub fn render_unanswered_questions<B: Backend>(
    app: &mut UnansweredQuestionsView,
    frame: &mut Frame<'_, B>,
) {
    let chunks = Layout::default()
        .constraints([Constraint::Length(9), Constraint::Min(8)].as_ref())
        .split(frame.size());

    frame.render_widget(
        Paragraph::new(
            "'k' - up\n\
             'j' - down\n\
             'r' - refresh (min 5min)\n\
             'n' - next page\n\
             'p' - previous page\n\
             'o' - open in browser\n\
             ' ' - toggle body\n\
                Press `Esc`, `Ctrl-C` or `q` to stop running.\n\
                Press left and right to increment and decrement the counter respectively.\n\
                ",
        )
        .block(
            Block::default()
                .title("Shortcuts")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::Cyan).bg(Color::Black))
        .alignment(Alignment::Left),
        chunks[0],
    );

    let questions: Vec<ListItem> = app
        .questions
        .items
        .iter()
        .enumerate()
        .map(|(i, q)| render_question(q, i))
        .collect();

    frame.render_stateful_widget(
        List::new(questions)
            .block(
                Block::default()
                    .title(format!("Unanswered Questions ({})", app.question_page))
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            )
            .style(Style::default().fg(Color::White).bg(Color::Black))
            //.highlight_style(Style::default().add_modifier(Modifier::BOLD))
            .highlight_symbol(">> "),
        chunks[1],
        &mut app.questions.state,
    );

    frame.render_stateful_widget(
        Scrollbar::default()
            .orientation(ScrollbarOrientation::VerticalRight)
            .symbols(scrollbar::VERTICAL)
            .begin_symbol(Some("↑"))
            .end_symbol(Some("↓")),
        chunks[1],
        &mut app.vertical_scroll_state,
    )
}

pub fn render_question_detail_view<B: Backend>(
    app: &mut QuestionReaderView,
    frame: &mut Frame<'_, B>,
) {
    let chunks = Layout::default()
        .constraints([Constraint::Length(9), Constraint::Min(8)].as_ref())
        .split(frame.size());

    frame.render_widget(
        Paragraph::new(
            "'k' - up\n\
             'j' - down\n\
             'o' - open in browser\n\
             ' ' - go to previous page\n\
                Press `Esc`, `Ctrl-C` or `q` to stop running.\n\
                Press left and right to increment and decrement the counter respectively.\n\
                ",
        )
        .block(
            Block::default()
                .title("Shortcuts")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::Cyan).bg(Color::Black))
        .alignment(Alignment::Left),
        chunks[0],
    );

    let paragraph = Paragraph::new(app.question.as_ref().unwrap().body.as_str())
        .style(Style::default().fg(Color::Gray))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::Gray))
                .title(Span::styled(
                    "Question",
                    Style::default().add_modifier(Modifier::BOLD),
                )),
        )
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true })
        .scroll((app.vertical_scroll_state, 0));

    frame.render_widget(paragraph, chunks[1]);
}
