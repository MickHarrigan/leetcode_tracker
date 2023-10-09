use anyhow::Result;
use ego_tree::NodeRef;
use lc_lib::common::{generate_request_client, query_endpoint, LeetCodeProblem, GQL_ENDPOINT};
use scraper::{Html, Node};
use strfmt::strfmt;
use tokio::runtime::{self, Builder};

use std::{
    collections::HashMap,
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

#[derive(Clone)]
pub struct StyleWrapper {
    pub modifier: Option<Modifier>,
    pub format: Option<String>,
    pub name: Option<String>,
}

impl std::ops::Add for StyleWrapper {
    type Output = StyleWrapper;
    fn add(self, rhs: Self) -> Self::Output {
        let mut out = StyleWrapper {
            format: None,
            modifier: None,
            name: None,
        };
        match (self.modifier, rhs.modifier) {
            (Some(parent), Some(child)) => out.modifier = Some(child | parent),
            (None, right) => out.modifier = right,
            (left, None) => out.modifier = left,
            // (None, None) => {}
        };
        let mut hash = HashMap::new();
        match (self.format, rhs.format) {
            (Some(parent), Some(child)) => {
                hash.insert("cont".to_string(), child);
                out.format = Some(strfmt(parent.as_str(), &hash).unwrap_or(parent));
            }
            (None, right) => out.format = right,
            (left, None) => out.format = left,
            // (None, None) => {}
        };
        out
    }
}

pub fn to_span(wrapper: &StyleWrapper, s: String) -> Span<'static> {
    let mut vars = HashMap::new();
    vars.insert("cont".to_string(), s.clone());
    // apply the style and format to s
    match (wrapper.modifier, &wrapper.format) {
        (Some(m), Some(f)) => Span::styled(
            strfmt(&f, &vars).unwrap_or(s.clone()),
            Style::default().add_modifier(m),
        ),
        (None, Some(f)) => Span::from(strfmt(&f, &vars).unwrap_or(s.clone())),
        (Some(m), None) => Span::styled(s.clone(), Style::default().add_modifier(m)),
        (None, None) => Span::from(s.clone()),
    }
}

fn style_from_name(name: &str) -> StyleWrapper {
    match name {
        "strong" => StyleWrapper {
            modifier: Some(Modifier::BOLD),
            format: None,
            name: Some("strong".to_string()),
        },
        "code" => StyleWrapper {
            modifier: Some(Modifier::ITALIC),
            // format: Some("`{cont}`".to_string()),
            format: None,
            name: Some("code".to_string()),
        },
        "li" => StyleWrapper {
            modifier: None,
            format: None,
            name: Some("li".to_string()),
        },
        "ul" => StyleWrapper {
            modifier: None,
            format: None,
            name: Some("ul".to_string()),
        },
        "em" => StyleWrapper {
            modifier: Some(Modifier::UNDERLINED),
            format: None,
            name: Some("em".to_string()),
        },
        "sup" => StyleWrapper {
            modifier: None,
            format: Some("^{cont}".to_string()),
            name: Some("em".to_string()),
        },
        _ => StyleWrapper {
            modifier: None,
            format: None,
            name: None,
        },
    }
}

pub fn condense_tree<'a>(root: &NodeRef<Node>) -> Text<'a> {
    // given a root, collect all children into a text that is passed upwards
    let mut text = Text::default();
    for child in root.children() {
        match child.value() {
            // condense helper must create a vec of spans
            Node::Element(element) => {
                if element.name() == "pre" || element.name() == "ul" {
                    // break each next section into lines at each \n
                    let wrapper = style_from_name(element.name());
                    text.lines
                        .extend(condense_tree_helper_preformatted(&child, wrapper));
                } else if element.name() != "font" {
                    let wrapper = style_from_name(element.name());
                    text.lines
                        .push(Line::from(condense_tree_helper(&child, wrapper)));
                }
            }
            Node::Text(words) => {
                text.lines.push(Line::from(words.text.to_string()));
            }
            _ => {}
        }
    }
    // text.lines.push(line);
    text
}

pub fn condense_tree_helper_preformatted<'a>(
    node: &NodeRef<Node>,
    parent_style: StyleWrapper,
) -> Vec<Line<'a>> {
    // this only applies to the preformatted text areas such as <pre> and <ul>
    let mut out: Vec<Line> = Vec::new();

    let mut curr = Line::default();
    for child in node.children() {
        match child.value() {
            Node::Text(words) => match words.text.chars().last() {
                Some(a) if a == '\n' => {
                    curr.spans.push(words.text.to_string().into());
                    out.push(curr);
                    curr = Line::default();
                }
                Some(a) => curr
                    .spans
                    .push(to_span(&parent_style, words.text.to_string())),
                None => {}
            },
            Node::Element(element) => {
                // let wrapper = parent_style.clone() + style_from_name(element.name());
                let wrapper = parent_style.clone() + style_from_name(element.name());
                curr.spans.extend(condense_tree_helper(&child, wrapper));
            }
            _ => {}
        }
    }
    if curr != Line::default() {
        out.push(curr);
    }
    let out: Vec<Line> = out
        .into_iter()
        .map(|line| {
            if parent_style.name == Some("ul".to_string())
                && line.spans.iter().nth(0) != Some(&Span::from("\n"))
            {
                [vec![Span::from("    \u{2022} ")], line.spans.clone()]
                    .concat()
                    .into()
            } else {
                line
            }
        })
        .collect();
    out
}

pub fn condense_tree_helper<'a>(node: &NodeRef<Node>, parent_style: StyleWrapper) -> Vec<Span<'a>> {
    // given a header element, iterates across each text element and collects them into a vector
    // that can be turned into a line at the caller
    let mut out: Vec<Span> = Vec::new();
    // iterate across the children and add them in
    for child in node.children() {
        match child.value() {
            Node::Element(element) => {
                let wrapper = parent_style.clone() + style_from_name(element.name());
                out.extend(condense_tree_helper(&child, wrapper));
            }
            Node::Text(words) => out.push(to_span(&parent_style, words.text.to_string())),
            _ => {}
        }
    }

    out
}

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

fn sanitize_html(contents: String) -> Text<'static> {
    // iterate across the string and find each tag to convert

    // removing \t gets rid of tabs that cause issues while parsing
    let contents = contents.replace("\t", "");
    let contents = contents.replace("&nbsp;", "");
    let frag = Html::parse_fragment(contents.as_str());

    let root = match frag.tree.root().children().next() {
        Some(a) => a,
        None => panic!("Incorrect HTML Passed!"),
    };
    condense_tree(&root)
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

        // TODO: Error handling!!!
        let data = rt.block_on(handle).unwrap().unwrap();

        // this is where the filling of the descriptions and code can occur
        let mut problems: Vec<LeetCodeProblem> = parse_problems(data, 50)
            .into_iter()
            .filter_map(Result::ok)
            .collect();

        let mut handles = Vec::with_capacity(50);

        for i in 0..50 {
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
            let mut lines = vec![Line::from(format!(
                "{} | {}. {}",
                match &app.problems.items[ind].status {
                    Some(a) => a,
                    None => &lc_lib::common::ProblemStatus::NotAttempted,
                },
                prob.frontend_question_id,
                prob.title.clone()
            ))];
            if ind == app.problems.state.selected().unwrap_or(0) {
                // show the tags in a list
                lines.push(Line::from("Empty for now! (Tags)"));
                // show the acceptance rate
                lines.push(Line::from(format!(
                    "{}",
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
