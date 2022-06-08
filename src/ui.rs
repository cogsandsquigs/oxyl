use crossterm::{
    event::{self, Event as CEvent, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use std::io;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};
use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{
        Block, BorderType, Borders, Cell, List, ListItem, ListState, Paragraph, Row, Table, Tabs,
    },
    Terminal,
};

use crate::db::{
	create_db,
	read_db,
	add_random_card_to_db,
	remove_card_at_index,
};

enum Event<I> {
    Input(I),
    Tick,
}

#[derive(Copy, Clone, Debug)]
enum MenuItem {
    Home,
	Study,
    Cards,
	Edit,
}

impl From<MenuItem> for usize {
    fn from(input: MenuItem) -> usize {
        match input {
            MenuItem::Home => 0,
            MenuItem::Study => 1,
			MenuItem::Cards => 2,
			MenuItem::Edit => 3,
        }
    }
}

pub fn render() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode().expect("can run in raw mode");

    let (tx, rx) = mpsc::channel();
    let tick_rate = Duration::from_millis(200);
    thread::spawn(move || {
        let mut last_tick = Instant::now();
        loop {
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if event::poll(timeout).expect("poll works") {
                if let CEvent::Key(key) = event::read().expect("can read events") {
                    tx.send(Event::Input(key)).expect("can send events");
                }
            }

            if last_tick.elapsed() >= tick_rate {
                if let Ok(_) = tx.send(Event::Tick) {
                    last_tick = Instant::now();
                }
            }
        }
    });

    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let menu_titles = vec!["Home", "Study", "Cards", "Add a card", "Delete a card", "Edit a card", "Quit"];
    let mut active_menu_item = MenuItem::Home;
    let mut card_list_state = ListState::default();
    card_list_state.select(Some(0));

    loop {
        terminal.draw(|rect| {
            let size = rect.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints(
                    [
                        Constraint::Length(3),
                        Constraint::Min(2),
                        Constraint::Length(3),
                    ]
                    .as_ref(),
                )
                .split(size);

            // the footer element
			/*
            let footer = Paragraph::new("oxyl")
                .style(Style::default().fg(Color::LightCyan))
                .alignment(Alignment::Center)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .style(Style::default().fg(Color::White))
                        .title("")
                        .border_type(BorderType::Plain),
                );
			*/

			// the menu element
            let menu = menu_titles
                .iter()
                .map(|t| {
                    let (first, rest) = t.split_at(1);
                    Spans::from(vec![
                        Span::styled(
                            first,
                            Style::default()
                                .fg(Color::Blue)
                                .add_modifier(Modifier::UNDERLINED),
                        ),
                        Span::styled(rest, Style::default().fg(Color::White)),
                    ])
                })
                .collect();

            let tabs = Tabs::new(menu)
                .select(active_menu_item.into())
                .block(Block::default().title("Menu").borders(Borders::ALL))
                .style(Style::default().fg(Color::White))
                .highlight_style(Style::default().fg(Color::Blue))
                .divider(Span::raw("|"));

            rect.render_widget(tabs, chunks[0]);
            match active_menu_item {
                MenuItem::Home => rect.render_widget(render_home(), chunks[1]),
				MenuItem::Study => {
					()
				},
				MenuItem::Cards => {
                    let cards_chunks = Layout::default()
                        .direction(Direction::Horizontal)
                        .constraints(
                            [Constraint::Percentage(30), Constraint::Percentage(70)].as_ref(),
                        )
                        .split(chunks[1]);
                    let (left, right) = render_cards(&mut card_list_state);
                    rect.render_stateful_widget(left, cards_chunks[0], &mut card_list_state);
                    rect.render_widget(right, cards_chunks[1]);
                },
				MenuItem::Edit => {
					()
				},
            }
            //rect.render_widget(footer, chunks[2]);
        })?;

        match rx.recv()? {
            Event::Input(event) => match event.code {
                KeyCode::Char('q') => {
                    disable_raw_mode()?;
                    terminal.show_cursor()?;
                    break;
                }
                KeyCode::Char('h') => active_menu_item = MenuItem::Home,
                KeyCode::Char('c') => active_menu_item = MenuItem::Cards,
                KeyCode::Char('a') => {
                    add_random_card_to_db().expect("can add new random card");
                }
                KeyCode::Char('d') => {
					if let Some(selected) = card_list_state.selected() {
						if read_db().expect("can't fetch card list").len() != 0 {
							if selected != 0 {
								card_list_state.select(Some(selected - 1));
							} else {
								card_list_state.select(Some(0));
							}
							remove_card_at_index(selected).expect("can't remove card");
						} else {
							card_list_state.select(None);
						}
				    }
                }
                KeyCode::Down => {
                    if let Some(selected) = card_list_state.selected() {
                        let amount_cards = read_db().expect("can fetch card list").len();
                        if selected >= amount_cards - 1 {
                            card_list_state.select(Some(0));
                        } else {
                            card_list_state.select(Some(selected + 1));
                        }
                    }
                }
                KeyCode::Up => {
                    if let Some(selected) = card_list_state.selected() {
                        let amount_cards = read_db().expect("can't fetch card list").len();
                        if selected > 0 {
                            card_list_state.select(Some(selected - 1));
                        } else {
                            card_list_state.select(Some(amount_cards - 1));
                        }
                    }
                }
                _ => {}
            },
            Event::Tick => {}
        }
    }
    Ok(())
}
// renders the cards, returns a widget to render
fn render_home<'a>() -> Paragraph<'a> {
    let home = Paragraph::new(vec![
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("Welcome")]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("to")]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::styled(
            "Oxyl!",
            Style::default().fg(Color::LightBlue),
        )]),
    ])
    .alignment(Alignment::Center)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .title("Home")
            .border_type(BorderType::Plain),
    );
    home
}

// renders the cards, returns two widgets to render
fn render_cards<'a>(card_list_state: &mut ListState) -> (List<'a>, Table<'a>) {
    let cards = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White))
        .title("Cards")
        .border_type(BorderType::Plain);

	// gets all the cards into a list
    let card_list = read_db().unwrap_or_else(|_| {
		create_db().expect("could not create a new db");
		read_db().expect("read db")
	});

	// cards into ListItems
    let items: Vec<_> = card_list
        .iter()
        .map(|card| {
            ListItem::new(Spans::from(vec![Span::styled(
				card.title(),
                Style::default(),
            )]))
        })
        .collect();

	// gets all the cards as ListItems into a list
	let list = List::new(items).block(cards).highlight_style(
        Style::default()
            .bg(Color::Blue)
            .fg(Color::Black)
            .add_modifier(Modifier::BOLD),
    );
	
	let mut table = Table::new(None);
	
	// gets the selected card if there are
	// cards present in the db
	if card_list.len() != 0 {
		let selected_card = card_list
		.get(
			card_list_state
			.selected()
			.unwrap_or_else(|| {
				card_list_state.select(Some(0));
				return 0;
			}),
		)
		.expect("no card exists")
		.clone();

		// set the data of the table
	    let card_detail = Table::new(vec![Row::new(vec![
	        Cell::from(Span::raw(selected_card.concept)),
				Cell::from(Span::raw(selected_card.front)),
				Cell::from(Span::raw(selected_card.back)),
	        Cell::from(Span::raw(selected_card.bucket.to_string())),
	    ])])
		// set the headers of the table
	    .header(Row::new(vec![
	        Cell::from(Span::styled(
	            "Concept",
	            Style::default().add_modifier(Modifier::BOLD),
	        )),
			Cell::from(Span::styled(
	            "Front",
	            Style::default().add_modifier(Modifier::BOLD),
	        )),
			Cell::from(Span::styled(
	            "Back",
	            Style::default().add_modifier(Modifier::BOLD),
	        )),
	        Cell::from(Span::styled(
	            "Bucket",
	            Style::default().add_modifier(Modifier::BOLD),
	        )),
	    ]))
		// set the block style (borders n stuff)
	    .block(
	        Block::default()
	            .borders(Borders::ALL)
	            .style(Style::default().fg(Color::White))
	            .title("Detail")
	            .border_type(BorderType::Plain),
	    )
	    .widths(&[
	        Constraint::Percentage(24),
			Constraint::Percentage(32),
			Constraint::Percentage(31),
	        Constraint::Percentage(13),
	    ]);

		table = card_detail;
	}
    
	// return the displayed widgets
    (list, table)
}