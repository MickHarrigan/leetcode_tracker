use std::{
    io::{self, stdout, Stdout},
    str::{self},
    time::{Duration, Instant},
};

use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{prelude::*, widgets::*};

pub struct StatefulList<T> {
    state: ListState,
    items: Vec<T>,
}

impl<T> StatefulList<T> {
    fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
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

    fn unselect(&mut self) {
        self.state.select(None);
    }
}

pub struct App<'a> {
    items: StatefulList<(&'a str, usize)>,
}

impl<'a> App<'a> {
    pub fn new() -> App<'a> {
        App {
            items: StatefulList::with_items(vec![
                ("Item0", 1),
                ("Item1", 2),
                ("Item2", 1),
                ("Item3", 3),
                ("Item4", 1),
                ("Item5", 4),
                ("Item6", 1),
                ("Item7", 3),
                ("Item8", 1),
                ("Item9", 6),
                ("Item10", 1),
                ("Item11", 3),
                ("Item12", 1),
                ("Item13", 2),
                ("Item14", 1),
                ("Item15", 1),
                ("Item16", 4),
                ("Item17", 1),
                ("Item18", 5),
                ("Item19", 4),
                ("Item20", 1),
                ("Item21", 2),
                ("Item22", 1),
                ("Item23", 3),
                ("Item24", 1),
            ]),
        }
    }
    
    pub fn on_tick(&mut self) {}

    pub fn run() -> io::Result<()> {
        let mut app = App::new();
        let mut last_tick = Instant::now();
        let tick_rate = Duration::from_millis(8);
        let mut terminal = init_terminal()?;
        loop {
            terminal.draw(|f| ui(&mut app, f))?;
            let timeout = tick_rate.saturating_sub(last_tick.elapsed());
            if event::poll(timeout)? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Char('h') => app.items.unselect(),
                        KeyCode::Char('j') => app.items.next(),
                        KeyCode::Char('k') => app.items.previous(),
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
    pub fn lorem_ipsum(&self) -> impl Widget {
        let text = vec![
            Line::from(format!("{}{}", "Lorem ipsum", "\u{25A0}")),
            Line::from(format!("{}{}", "Lorem ipsum", "\u{25FC}")),
            Line::from(format!("{}{}", "Lorem ipsum", "\u{2BC0}")),
            Line::from(format!("{}{}", "Lorem ipsum", "\u{25FE}")),
            Line::from(format!("{}{}", "Lorem ipsum", "\u{2B1B}")),
            Line::from(format!("{}{}", "Lorem ipsum", "\u{20DE}")),
            Line::from(format!("{}{}", "Lorem ipsum", "\u{2705}")),
            Line::from(format!("{}{}", "Lorem ipsum", "\u{1FBB1}")),
            Line::from(format!("{}{}", "Lorem ipsum", "\u{1FBC0}")),
            Line::from(format!("{}{}", "Lorem ipsum", "\u{1FBC4}")),
        ];
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
        .items
        .items
        .iter()
        .map(|(name, val)| {
            let mut lines = vec![Line::from(*name)];
            for _ in 0..*val {
                lines.push(
                    "Description of the problem or something, i don't really know"
                        .italic()
                        .into(),
                );
            }
            ListItem::new(lines).style(Style::default().fg(Color::Yellow).bg(Color::LightGreen))
        })
        .collect();
    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Problem List"))
        .highlight_style(
            Style::default()
                .bg(Color::Blue)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    let lorem_ipsum = app.lorem_ipsum();

    f.render_stateful_widget(list, chunks[0], &mut app.items.state);
    f.render_widget(lorem_ipsum, chunks[1]);
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
