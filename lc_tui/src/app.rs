use anyhow::Result;
use lc_lib::common::{generate_request_client, query_endpoint, LeetCodeProblem, GQL_ENDPOINT};
use serde_json::Value;
use tokio::runtime::{self, Builder};

use std::{
    io::{self, stdout, Stdout},
    str::{self, FromStr},
    time::{Duration, Instant},
};

use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{prelude::*, widgets::*};
use reqwest::Url;

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

pub struct App {
    /// `problems` will be either filled in from a local cache or fetched by
    /// reqwest to update the cache.
    /// The location for caching will be primarily "~/.cache/lc/".
    problems: StatefulList<LeetCodeProblem>,
}

fn parse_problems(json: serde_json::Value, count: usize) -> Vec<Result<LeetCodeProblem>> {
    let list = &json["data"]["problemsetQuestionList"];
    let mut out = Vec::with_capacity(count);
    for i in 0..count {
        out.push(
            serde_json::from_value::<LeetCodeProblem>(list["questions"][i].clone())
                .map_err(|e| e.into()),
        );
    }
    out
}

/// This takes a problem and updates the description, code snippet, and maybe tests
fn update_problem(prob: &mut LeetCodeProblem, json: serde_json::Value) {
    // clean up the json,
    // get the description as raw
    // get the code as raw
    // update the prob
    println!("{}", json);
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
        let link = Url::from_str("https://leetcode.com/problems/all").unwrap();
        let client = generate_request_client(&link).unwrap();
        let handle = rt.spawn(query_endpoint(
            GQL_ENDPOINT.to_string(),
            query,
            client.clone(),
        ));

        // HERE: is where the cache checking could happen

        let data = rt.block_on(handle).unwrap().unwrap();

        // this is where the filling of the descriptions and code can occur
        let mut problems: Vec<LeetCodeProblem> = parse_problems(data, 50)
            .into_iter()
            .filter_map(Result::ok)
            .collect();

        let mut handles = Vec::with_capacity(10);

        for i in 0..10 {
            let slug = problems[i].title_slug.clone();
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
    pub fn disp_problem_description(&self) -> impl Widget {
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

        Paragraph::new(text).block(
            Block::default()
                .borders(Borders::ALL)
                .style(
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD),
                )
                .title("Problem Description"),
        )
    }
}

fn ui<B: Backend>(app: &mut App, f: &mut Frame<B>) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(f.size());

    let items: Vec<ListItem> = app
        .problems
        .items
        .iter()
        .enumerate()
        .map(|(ind, prob)| {
            let mut lines = vec![Line::from(prob.title.clone())];
            lines.push(prob.frontend_question_id.italic().into());
            // TODO: make the selected have more lines in it
            match app.problems.state.selected() {
                Some(i) if i == ind => {}
                None => {}
                _ => {}
            }
            ListItem::new(lines).style(Style::default().fg(Color::White).bg(Color::default()))
        })
        .collect();
    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Problem List"))
        .highlight_style(
            Style::default()
                .bg(Color::LightBlue)
                .fg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    let description = app.disp_problem_description();

    f.render_stateful_widget(list, chunks[0], &mut app.problems.state);
    f.render_widget(description, chunks[1]);
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
