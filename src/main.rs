mod types;
mod game;
use std::{sync::Arc, fmt::format};

use crate::types::nums::BeegNum;


/*fn main() {

    let mut game = game::GameStruct::blank();

    let mut start = std::time::Instant::now();

    let mut prevbalance = game.player.balance.value.clone();

    loop {

        // If it's been a second, process a tick
        if start.elapsed().as_secs_f64() >= 1.0 {


            game.player.balance += BeegNum::new(game.money_process(start.elapsed()) as u128, 0);

            for tier in game.upgrades.clone() {

                for upgrade in tier {
                    // If we can buy the upgrade, buy it
                    let canbuy = game.can_buy_upgrade(upgrade.clone());
                    if canbuy.0 {
                        
                        game.buy_upgrade(upgrade.clone());

                        println!("Bought Upgrade {} {} for {}$, now gives {}x bonus", upgrade.name, upgrade.desc, canbuy.1, upgrade.multiplier * canbuy.2 as f64);
                    }
                } 
            }


            // Restart the tick timer
            start = std::time::Instant::now();


        }

        if game.player.balance.value != prevbalance {
            println!("{}$", game.player.balance.value);
        }

        prevbalance = game.player.balance.value;
        
    }
}*/


use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use game::GameStruct;
use std::{
    error::Error,
    io,
    time::{Duration, Instant},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout, Corner},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph, Wrap, ListItem, List, ListState},
    Frame, Terminal,
};

struct StatefulList<T> {
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

struct App<'a> {
    game: &'a mut GameStruct,
    upgrades: StatefulList<game::Upgrade>
}

impl<'a> App<'a> {
    fn new(game: &'a mut GameStruct) -> App<'a> {
        App {
            game: game,
            upgrades: App::new_upgrade_list()
        }
    }

    fn on_tick(&mut self) {
        // Tui tick
    }

    // Generates a stateful list from the current list
    fn generate_upgrade_list(&self) -> StatefulList<game::Upgrade> {
        StatefulList { state: self.upgrades.state.clone(), items: game::GameStruct::get_all_upgrades()}
    }

    // Generates and updates the current list of upgrades
    fn update_upgrade_list(&mut self) {
        self.upgrades = self.generate_upgrade_list();
    }

    // Generates a new stateful list from a gamestruct
    fn new_upgrade_list() -> StatefulList<game::Upgrade>{
        StatefulList { state: ListState::default(), items: game::GameStruct::get_all_upgrades() }
    }

}

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let tick_rate = Duration::from_millis(250);

    let mut game = game::GameStruct::blank();
    let app = App::new(&mut game);
    let res = run_app(&mut terminal, app, tick_rate);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
    tick_rate: Duration,
) -> io::Result<()> {
    let mut last_tick = Instant::now();

    let mut start = std::time::Instant::now();


    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {

                // Inbut handler
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Left => app.upgrades.unselect(),
                    KeyCode::Down => app.upgrades.next(),
                    KeyCode::Up => app.upgrades.previous(),
                    _ => {}
                }

            }
        }
        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = Instant::now();
        }

        let mut game = &mut app.game;

        // If it's been a second, process a tick
        if start.elapsed().as_secs_f64() >= 1.0 {


            game.player.balance += BeegNum::new(game.money_process(start.elapsed()) as u128, 0);

            /*for tier in game.upgrades.clone() {

                for upgrade in tier {
                    // If we can buy the upgrade, buy it
                    let canbuy = game.can_buy_upgrade(upgrade.clone());
                    if canbuy.0 {
                        
                        game.buy_upgrade(upgrade.clone());

                        println!("Bought Upgrade {} {} for {}$, now gives {}x bonus", upgrade.name, upgrade.desc, canbuy.1, upgrade.multiplier * canbuy.2 as f64);
                    }
                } 
            }*/


            // Restart the tick timer
            start = std::time::Instant::now();

        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let size = f.size();

    let block = Block::default().style(Style::default());
    f.render_widget(block, size);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Percentage(75),
            ]
            .as_ref(),
        )
        .split(size);

    // Iterate through all elements in the `items` app and append some debug text to it.
    let items: Vec<ListItem> = app
        .upgrades
        .items
        .iter()
        .map(|i| {
            let mut lines = vec![Spans::from(Span::styled(i.name.clone(), 
            Style::default().fg(Color::Rgb(235, 219, 178)).add_modifier(Modifier::BOLD)))
        ];

            // Push all the necessary info as styled spans

            // Description
            lines.push(Spans::from(Span::styled(
                format!(r#""{}""#, i.desc),
                Style::default().fg(Color::Rgb(235, 219, 178)).add_modifier(Modifier::ITALIC),
            )));

            // How many we have
            lines.push(Spans::from(Span::styled(
                format!(r#"({}/{})"#, 0, i.tiers),
                Style::default().fg(Color::Rgb(213, 196, 161)), // Gruvbox fg 2
            )));

            // Cost 
            // TODO: Show the actual cost
            lines.push(Spans::from(Span::styled(
                format!(r#"{}$"#, i.base_cost),
                Style::default().fg(Color::Rgb(184, 187, 38)), // Gruvbox green
            )));

            // Small divider
            lines.push(Spans::from(Span::styled(
                format!("-----------"),
                Style::default().fg(Color::Rgb(213, 196, 161)) // Gruvbox fg 2
            )));

    
            ListItem::new(lines).style(Style::default())
        })
        .collect();

    // Create a List from all list items and highlight the currently selected one
    let items = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("List"))
        .highlight_style(
            Style::default()
                .bg(Color::Rgb(69, 133, 136)) // Gruvbox blue
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    // We can now render the item list
    f.render_stateful_widget(items, chunks[1], &mut app.upgrades.state);

    let text = vec![
        Spans::from(Span::styled(format!("{}$", app.game.player.balance.value),
        Style::default()
        .fg(Color::Rgb(184, 187, 38) // Gruvbox green
    )))
    ];

    let create_block = |title| {
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default())
            .title(Span::styled(
                title,
                Style::default().add_modifier(Modifier::BOLD),
            ))
    };
    let paragraph = Paragraph::new(text.clone())
        .style(Style::default())
        .block(create_block(""))
        .alignment(Alignment::Center);
    f.render_widget(paragraph, chunks[0]);
    /*let paragraph = Paragraph::new("")
        .style(Style::default())
        .block(create_block("Upgrades"))
        .alignment(Alignment::Center);
    f.render_widget(paragraph, chunks[1]);*/
}