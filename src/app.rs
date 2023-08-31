use std::error;
use webbrowser;

use ratatui::widgets::{ListState, ScrollbarState};

use crate::stack::{self, from_html};

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
pub struct Question {
    pub title: String,
    pub link: String,
    pub body: String,
    pub tags: Vec<String>,
    pub answer_count: u32,
    pub description: String,
    pub show_body: bool,
}

/// Model for scrollable list
#[derive(Debug)]
pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>,
}

impl<T> StatefulList<T> {
    fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items,
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn unselect(&mut self) {
        self.state.select(None);
    }
}

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    pub questions: StatefulList<Question>,
    pub question_page: u8,
    stack_overflow_client: stack::StackOverflowClient,
    pub vertical_scroll_state: ScrollbarState,
    pub vertical_scroll_position: usize,
}

impl Default for App {
    fn default() -> Self {
        let mut vertical_scroll_state = ScrollbarState::default();
        vertical_scroll_state = vertical_scroll_state.content_length(2);
        vertical_scroll_state = vertical_scroll_state.viewport_content_length(1);

        Self {
            running: true,
            // Create default value for questions
            question_page: 1,
            questions: StatefulList::with_items(vec![
                Question {
                    title: "How to do X?".to_string(),
                    link: "https://stackoverflow.com/questions/12345".to_string(),
                    tags: vec!["rust".to_string(), "python".to_string()],
                    body: from_html("<p>The problem is how to open and close the details and summary tag according to array index in svelte js. I want that details will be closed when clicked on another details element.</p>\n<p>I tried binding open attribute of details element but it will toggle all the details elements which are created with each loop in svelte. I am expecting it will open and close according to its array index.</p>\n<pre class=\"lang-html prettyprint-override\"><code>&lt;script&gt;\n    let name = 'world';\n    let isOpen = true;\n&lt;/script&gt;\n\n&lt;h1&gt;\n    The details is {isOpen ? 'open' : 'not open'}\n&lt;/h1&gt;\n{#each Array(10)as _}\n&lt;details bind:open={isOpen}&gt;\n    &lt;summary&gt;Details&lt;/summary&gt;\n    &lt;p&gt;\n        Something small enough to escape casual notice.\n    &lt;/p&gt;\n&lt;/details&gt;\n{/each}\n</code></pre>\n"),
                    answer_count: 1,
                    description: "This is a description".to_string(),
                    show_body: false,
                },
                Question {
                    title: "How to do Y?".to_string(),
                    link: "https://stackoverflow.com/questions/12345".to_string(),
                    tags: vec!["rust".to_string(), "python".to_string()],
                    body:  "<p>I have those two datasets:</p>\n<ol>\n<li>about 800 values from past three days with timestamps during those days</li>\n<li>3 values from past three days with timestamp at midnight</li>\n</ol>\n<p>Is it somehow possible to display those two lines in one graph, but second line respecting timestamps of line 1)? When I read documentation, there is written that all datasets should have same number of labels, but then I came across some solutions that might do what I want. But those solutions seems to be for previous version of chart.js and no longer work.</p>\n<p>Or is the only solution to modify dataset 2) to interpolate value for every point in dataset 1)\nThanks a lot</p>\n<p>Now when I draw chart, 1) dataset is drawn correctly and second dataset is obviously displayed as small dot at the beginning since it thinks that those three values are for first 3 timestamps of dataset 1):\n<a href=\"https://i.stack.imgur.com/FYCck.png\" rel=\"nofollow noreferrer\">enter image description here</a></p>\n<p>What I want is this:\n<a href=\"https://i.stack.imgur.com/Moy9I.png\" rel=\"nofollow noreferrer\">enter image description here</a></p>\n".to_string(),
                    answer_count: 0,
                    description: "This is a description".to_string(),
                    show_body: false,
                },
                Question {
                    title: "How to do Z?".to_string(),
                    link: "https://stackoverflow.com/questions/12345".to_string(),
                    tags: vec!["rust".to_string(), "python".to_string()],
                    body: "This is a description".to_string(),
                    answer_count: 0,
                    description: "This is a description".to_string(),
                    show_body: false,
                },
            ]),
            stack_overflow_client: stack::StackOverflowClient::default(),
            vertical_scroll_state,
            vertical_scroll_position: 1,
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        let mut default = Self::default();
        default.refresh_unanswered_questions();
        default
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn refresh_unanswered_questions(&mut self) {
        let questions = self
            .stack_overflow_client
            .get_unanswered_questions(self.question_page)
            .expect("Failed to get unanswered questions");

        self.questions = StatefulList::with_items(questions);

        self.vertical_scroll_state.first();

        self.vertical_scroll_state = self
            .vertical_scroll_state
            .content_length((self.questions.items.len() - 1) as u16);
    }

    pub fn next_question_page(&mut self) {
        self.question_page += 1;

        self.refresh_unanswered_questions();
    }

    pub fn previous_question_page(&mut self) {
        if self.question_page > 1 {
            self.question_page -= 1;
            self.refresh_unanswered_questions();
        }
    }

    pub fn next_unanswered_question(&mut self) {
        self.questions.next();
        self.vertical_scroll_state = self
            .vertical_scroll_state
            .position(self.questions.state.selected().unwrap_or(0) as u16);
    }

    pub fn previous_unanswered_question(&mut self) {
        self.questions.previous();
        self.vertical_scroll_state = self
            .vertical_scroll_state
            .position(self.questions.state.selected().unwrap_or(0) as u16);
    }

    pub fn open_selected_question(&mut self) {
        let question = &self.questions.items[self.questions.state.selected().unwrap() as usize];

        if let Err(e) = webbrowser::open(question.link.as_str()) {
            eprintln!("Failed to open link: {}", e);
        }
    }

    pub fn toogle_question_body(&mut self) {
        let question = &mut self.questions.items[self.questions.state.selected().unwrap() as usize];
        question.show_body = !question.show_body;
    }
}
