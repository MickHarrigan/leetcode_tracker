use anyhow::Result;
use lc_lib::common::{
    generate_request_client, get_lc_dir, query_endpoint, LeetCodeProblem, GQL_ENDPOINT,
};
use syntect::{
    easy::HighlightLines,
    highlighting::{Theme, ThemeSet},
    parsing::{SyntaxReference, SyntaxSet},
    util::LinesWithEndings,
};
use syntect_tui::into_span;
use tokio::runtime::{self, Builder};

use std::{
    fs,
    io::{self, stdout, Stdout},
    path::{Path, PathBuf},
    str::FromStr,
    time::{Duration, Instant},
};

use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    prelude::*,
    widgets::{canvas::Canvas, *},
};
use reqwest::Url;

use crate::ui::sanitize_html;

pub struct StatefulList<T> {
    state: ListState,
    items: Vec<T>,
}

impl<T> StatefulList<T> {
    fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default().with_selected(Some(0)),
            items,
        }
    }

    fn next(&mut self) {
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

    fn previous(&mut self) {
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

    // fn unselect(&mut self) {
    //     self.state.select(None);
    // }
}

pub struct CodeWindow {
    // maybe a vec at a later time, if there is a need for 3 or more items
    // titles: Vec<Option<String>>,
    titles: (Option<String>, Option<String>),
    index: usize,
}
impl CodeWindow {
    pub fn alternate(&mut self) {
        self.index = if self.index == 0 { 1 } else { 0 }
    }
    // pub fn increment(&mut self) {
    //     // this should change to the next item in the tab list
    //     self.index = if self.index == 0 { 1 } else { 0 }
    // }
    // pub fn decrement(&mut self) {
    //     if self.index == 0 {
    //         self.index = self.titles.len() - 1;
    //     } else {
    //         self.index -= 1;
    //     }
    // }
}

pub struct App {
    /// `problems` will be either filled in from a local cache or fetched by
    /// reqwest to update the cache.
    /// The location for caching will be primarily "~/.cache/lc/".
    problems: StatefulList<LeetCodeProblem>,
    code_window: CodeWindow,

    // these are only required for the highlighting of the problems
    // if these have to move then thats not a problem
    ps: SyntaxSet,
    ts: ThemeSet,
}

fn parse_problems(json: &serde_json::Value, count: usize) -> Vec<Result<LeetCodeProblem>> {
    match json
        .get("data")
        .and_then(|a| a.get("problemsetQuestionList"))
        .and_then(|b| b.get("questions"))
    {
        Some(questions) => {
            let mut out = Vec::with_capacity(count);
            for i in 0..count {
                out.push(parse_problem(&questions[i]));
            }
            out
        }
        None => vec![],
    }
}
fn parse_problem(problem_json: &serde_json::Value) -> Result<LeetCodeProblem> {
    serde_json::from_value::<LeetCodeProblem>(problem_json.to_owned()).map_err(|e| e.into())
}

/// This takes a problem and updates the description, code snippet, and maybe tests
fn update_problem(prob: &mut LeetCodeProblem, json: serde_json::Value) {
    // clean up the json,
    // get the description as raw
    // get the code as raw
    // update the prob
    match json
        .get("data")
        .and_then(|val| val.get("question"))
        .and_then(|val| val.get("codeSnippets"))
        .and_then(|val| {
            val.as_array().and_then(|list| {
                list.iter()
                    .find(|snip| {
                        snip.get("langSlug").and_then(|lang| lang.as_str()) == Some("rust")
                    })
                    .and_then(|snippet| snippet.get("code").and_then(|code| code.as_str()))
            })
        }) {
        Some(snip) => prob.snippet = snip.to_owned(),
        None => prob.snippet = "".to_owned(),
    };
    match json
        .get("data")
        .and_then(|val| val.get("question"))
        .and_then(|val| val.get("content"))
        .and_then(|cont| cont.as_str())
    {
        Some(cont) => prob.description = cont.to_owned(),
        None => prob.description = "".to_owned(),
    };
}

impl App {
    pub fn new(rt: runtime::Runtime) -> App {
        // upon startup new should first check the cache for existing problem info
        // and if it cannot find anything, then it reaches out to the gql server.

        // first query the server
        // with the data that it returns iterate over the questions and generate a new
        // `LeetCodeProblem` from each

        let query = serde_json::json!({
            "query":"query problemsetQuestionList($categorySlug: String, $limit: Int, $skip: Int, $filters: QuestionListFilterInput) {
                problemsetQuestionList: questionList(
                    categorySlug: $categorySlug
                    limit: $limit
                    skip: $skip
                    filters: $filters
                    ) {
                        total: totalNum
                        questions: data {
                            acRate
                            difficulty
                            frontendQuestionId: questionFrontendId
                            status
                            title
                            titleSlug
                            topicTags {
                                name
                                id
                                slug
                            }
                        }
                }
            }",
            "variables":{"categorySlug":"","skip":0,"limit":50,"filters":{}},
            "operationName":"problemsetQuestionList"
        });
        let query_of_the_day = serde_json::json!({
            "query":"query questionOfToday {
                activeDailyCodingChallengeQuestion {
                    date
                    question {
                        acRate
                        difficulty
                        frontendQuestionId: questionFrontendId
                        status
                        title
                        titleSlug
                        topicTags {
                            name
                            id
                            slug
                        }
                    }
                }
            }",
            "variables":{},
            "operationName":"questionOfToday"
        });
        let link = Url::from_str("https://leetcode.com/problemset/all").unwrap();
        let client = generate_request_client(&link).unwrap();
        let handle = rt.spawn(query_endpoint(
            GQL_ENDPOINT.to_string(),
            query,
            client.clone(),
        ));

        // HERE: is where the cache checking could happen

        // TODO: Error handling!!!
        let data = rt.block_on(handle).unwrap().unwrap();

        // this is where the filling of the descriptions and code can occur
        let mut problems: Vec<LeetCodeProblem> = parse_problems(&data, 50)
            .into_iter()
            .filter_map(Result::ok)
            .collect();
        let handle_of_the_day = rt.spawn(query_endpoint(
            GQL_ENDPOINT.to_string(),
            query_of_the_day,
            client.clone(),
        ));

        let data_of_the_day = rt.block_on(handle_of_the_day).unwrap().unwrap();
        // REPLACE ALL THE UNWRAPS
        let problem_of_the_day = match data_of_the_day
            .get("data")
            .and_then(|a| a.get("activeDailyCodingChallengeQuestion"))
            .and_then(|a| a.get("question"))
        {
            Some(problem) => parse_problem(problem),
            None => Ok(LeetCodeProblem::default()),
        };
        let problem_of_the_day = problem_of_the_day.unwrap();
        problems.insert(0, problem_of_the_day);

        // now that the problem of the day is here, move it to the front

        let mut handles = Vec::with_capacity(51);

        for i in 0..51 {
            let slug = &problems[i].title_slug;
            let query = serde_json::json!({
                "query": "query questionContent($titleSlug: String!) {
                    question(titleSlug: $titleSlug) {
                        content
                        codeSnippets {
                            langSlug
                            code
                        }
                    }
                }",
                "variables":{"titleSlug":slug},
                "operationName":"questionContent"
            });
            handles.push((
                i,
                rt.spawn(query_endpoint(
                    GQL_ENDPOINT.to_string(),
                    query.clone(),
                    client.clone(),
                )),
            ));
        }

        for (ind, handle) in handles {
            let data = rt.block_on(handle).unwrap().unwrap();
            update_problem(&mut problems[ind], data);
        }

        // parse the data into a vec![LeetCodeProblem]
        // 50 is the amount of problems to read in at one time

        App {
            problems: StatefulList::with_items(problems),
            code_window: CodeWindow {
                titles: (Some("Provided Code".to_string()), None),
                index: 0,
            },
            ps: SyntaxSet::load_defaults_newlines(),
            ts: ThemeSet::load_from_folder("/home/mick/Dev/Rust/themes").unwrap(),
        }
    }

    pub fn on_tick(&mut self) {}

    pub fn run() -> io::Result<()> {
        let rt = Builder::new_multi_thread()
            .worker_threads(1)
            .enable_all()
            .build()
            .unwrap();
        let mut app = App::new(rt);
        let mut last_tick = Instant::now();
        let tick_rate = Duration::from_millis(16);
        let mut terminal = init_terminal()?;
        loop {
            terminal.draw(|f| ui(&mut app, f))?;
            let timeout = tick_rate.saturating_sub(last_tick.elapsed());
            if event::poll(timeout)? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char('q') => break,
                        // KeyCode::Char('h') => app.problems.unselect(),
                        KeyCode::Char('j') => app.problems.next(),
                        KeyCode::Char('k') => app.problems.previous(),
                        KeyCode::Tab => app.code_window.alternate(),
                        // KeyCode::Enter => edit::edit_file("file_path_in_repo").unwrap(),
                        _ => {}
                    }
                }
            }
            if last_tick.elapsed() >= tick_rate {
                app.on_tick();
                last_tick = Instant::now();
            }
        }
        restore_terminal()
    }
    pub fn disp_provided_code(&self) -> impl Widget + '_ {
        // if self.code_window.index == 0

        let text = match self.code_window.index {
            0 => {
                let code = match self.problems.state.selected() {
                    Some(i) => self.problems.items.iter().nth(i).unwrap().snippet.clone(),
                    None => "".to_string(),
                };
                let syntax = self.ps.find_syntax_by_extension("rs").unwrap();
                let theme = &self.ts.themes["Catppuccin-macchiato"];
                let mut h = HighlightLines::new(syntax, &theme);
                let mut text = Text::default();
                for line in LinesWithEndings::from(&code).into_iter() {
                    let line_spans: Vec<Span> = h
                        .highlight_line(line, &self.ps)
                        .unwrap()
                        .into_iter()
                        .filter_map(|segment| into_span(segment).ok())
                        .map(|a| Span::styled(a.content.into_owned(), a.style))
                        .collect();
                    let rat_line = Line::from(line_spans);
                    text.lines.push(rat_line);
                }
                text
            }
            1 => {
                // read the file and show that contents
                // check LEETCODE_DIR/src/<problem.frontend_question_id>/src/main.rs
                // if the file exists then it shows something
                // Otherwise it shouldn't even be able to have this tab
                let problem_number = &self.problems.items
                    [self.problems.state.selected().unwrap_or(0)]
                .frontend_question_id;
                let lc_dir = match get_lc_dir() {
                    Ok(a) => a,
                    Err(e) => panic!("{}", e),
                };

                let path = PathBuf::from(format!("{}src/{}/src/main.rs", lc_dir, problem_number));
                if path.exists() {
                    let code = match fs::read_to_string(path) {
                        Ok(a) => a,
                        Err(e) => panic!("{}", e),
                    };

                    let syntax = self.ps.find_syntax_by_extension("rs").unwrap();
                    let theme = &self.ts.themes["Catppuccin-macchiato"];
                    let mut h = HighlightLines::new(syntax, &theme);
                    let mut text = Text::default();
                    for line in LinesWithEndings::from(&code).into_iter() {
                        let line_spans: Vec<Span> = h
                            .highlight_line(line, &self.ps)
                            .unwrap()
                            .into_iter()
                            .filter_map(|segment| into_span(segment).ok())
                            .map(|a| Span::styled(a.content.into_owned(), a.style))
                            .collect();
                        let rat_line = Line::from(line_spans);
                        text.lines.push(rat_line);
                    }
                    text
                } else {
                    Text::from("Problem has not yet been attempted!".to_string())
                }
            }
            _ => unreachable!(),
        };
        // if self.code_window.index == otherwise
        Paragraph::new(text).wrap(Wrap { trim: false }).block(
            Block::default()
                .borders(Borders::ALL)
                .title("Provided Code"),
        )
    }
    pub fn disp_problem_description(&self) -> impl Widget + '_ {
        let text = match self.problems.state.selected() {
            Some(i) => self
                .problems
                .items
                .iter()
                .nth(i)
                .unwrap()
                .description
                .clone(),
            None => "".to_owned(),
        };

        let text = sanitize_html(text);
        Paragraph::new(text).wrap(Wrap { trim: false }).block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default())
                .title("Problem Description"),
        )
    }
}

fn ui<B: Backend>(app: &mut App, f: &mut Frame<B>) {
    let panels = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(f.size());

    let right_panel = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(panels[1]);

    let items: Vec<ListItem> = app
        .problems
        .items
        .iter()
        .enumerate()
        .map(|(ind, prob)| {
            let mut lines = vec![Line::from(vec![
                Span::styled(
                    format!(
                        "{}",
                        match &app.problems.items[ind].status {
                            Some(a) => a,
                            None => &lc_lib::common::ProblemStatus::NotAttempted,
                        }
                    ),
                    Style::default().fg(match &app.problems.items[ind].status {
                        Some(lc_lib::common::ProblemStatus::Accepted) => Color::Green,
                        Some(lc_lib::common::ProblemStatus::Attempted) => Color::Magenta,
                        _ => Color::Reset,
                    }),
                ),
                Span::raw(" | "),
                Span::styled(
                    format!("{}. {}", prob.frontend_question_id, &prob.title),
                    match &prob.difficulty {
                        lc_lib::common::ProblemDifficulty::Easy => {
                            Style::default().fg(Color::Green)
                        }
                        lc_lib::common::ProblemDifficulty::Medium => {
                            Style::default().fg(Color::Yellow)
                        }
                        lc_lib::common::ProblemDifficulty::Hard => Style::default().fg(Color::Red),
                    },
                ),
            ])];
            if ind == app.problems.state.selected().unwrap_or(0) {
                // show the tags in a list
                lines.push(Line::from("    Tags: "));
                // show the acceptance rate
                lines.push(Line::from(format!(
                    "    Acceptance Rate: {:.2}%",
                    app.problems.items[ind].acceptance_rate
                )));
            }
            ListItem::new(lines).style(Style::default().fg(Color::White).bg(Color::default()))
        })
        .collect();
    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Problem List"))
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    let description = app.disp_problem_description();
    let provided_code = app.disp_provided_code();

    f.render_widget(description, right_panel[0]);
    f.render_widget(provided_code, right_panel[1]);
    f.render_stateful_widget(list, panels[0], &mut app.problems.state);
}

fn init_terminal() -> io::Result<Terminal<CrosstermBackend<Stdout>>> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    Terminal::new(CrosstermBackend::new(stdout()))
}

fn restore_terminal() -> io::Result<()> {
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}
