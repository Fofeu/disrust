#![allow(unused_imports)]
//GUI = gooey
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Spans,
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph},
    Frame, Terminal,
};

use std::time::Duration;
use std::time::Instant;

use std::io;

use anyhow::Result;

use crate::api::gateway_thread;

use crate::ui::app::App;

pub async fn summon_gooey() -> anyhow::Result<()> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    // let mut cbox = ChatBox::new();
    let response = run_app(&mut terminal, &mut app).await;

    // restore terminal. Closes the program
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    response
}

//Main loop
async fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> anyhow::Result<()> {
    // let tick_rate = Duration::from_millis(250);
    let last_tick = Instant::now();
    loop {
        terminal.draw(|f| ui(f, app))?;

        if last_tick.elapsed() >= Duration::from_secs(5) {
            break;
        }
        // match gate_rx.try_recv() {
        //     Ok(v) => app.react_to_gateway(&v),
        //     Err(_v) => {}
        // }

        // //Draws the screen. Comment out when debugging
        // terminal.draw(|f| ui(f, app, cbox))?;
        // let timeout = tick_rate
        //     .checked_sub(last_tick.elapsed())
        //     .unwrap_or_else(|| Duration::from_secs(0));

        // //Read input
        // //CodeAesthetic would be upset
        // //Have to use poll to avoid blocking
        // if crossterm::event::poll(timeout)? {
        //     if let Event::Key(key) = event::read()? {
        //         match cbox.input_mode {
        //             InputMode::Normal => match key.code {
        //                 KeyCode::Char('q') => return Ok(()),
        //                 KeyCode::Char('e') => cbox.toggle(),
        //                 KeyCode::Left => app.unselect(),
        //                 KeyCode::Down => app.next().await,
        //                 KeyCode::Up => app.previous().await,
        //                 KeyCode::Enter => app.enter_guild(),
        //                 KeyCode::Esc => app.leave_guild(),
        //                 _ => (),
        //             },
        //             InputMode::Editing => match key.code {
        //                 KeyCode::Enter => cbox.send_message(app).await,
        //                 KeyCode::Esc => cbox.toggle(),
        //                 KeyCode::Char(c) => cbox.input.push(c),
        //                 KeyCode::Backspace => {
        //                     cbox.input.pop();
        //                 }
        //                 _ => (),
        //             },
        //         }
        //     }
        // }
        // if last_tick.elapsed() >= tick_rate {
        //     last_tick = Instant::now();
        // }
    }
    Ok(())
}

//Maybe make each block a function
//Sets up how the ui looks like
fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    //Wrapping block
    //Mandatory margin of 1+
    let wrapping_block = Block::default()
        .borders(Borders::ALL)
        .title("Disrust")
        .title_alignment(Alignment::Left)
        .border_type(BorderType::Thick);
    f.render_widget(wrapping_block, f.size());

    // // this is all just defined boundaries used when drawing
    // let chunks = Layout::default()
    //     .direction(Direction::Horizontal)
    //     .margin(1)
    //     .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
    //     .split(f.size());

    // let right_chunks = Layout::default()
    //     .direction(Direction::Vertical)
    //     .constraints([Constraint::Percentage(94), Constraint::Percentage(6)].as_ref())
    //     .split(chunks[1]);

    // // Create the channels part
    // let mut items = guilds_to_listitems(&app.guilds.items);
    // match app.mode {
    //     GuildMode => {}
    //     ChannelMode => {
    //         items = channels_to_listitems(&app.items.items);
    //     }
    // }

    // let items = List::new(items)
    //     .block(
    //         Block::default()
    //             .borders(Borders::ALL)
    //             .title("Guilds and Channels"),
    //     )
    //     .highlight_style(
    //         Style::default()
    //             .bg(Color::LightGreen)
    //             .add_modifier(Modifier::BOLD),
    //     )
    //     .highlight_symbol(">> ");

    // // We can now render the item list
    // // Displays channels or guilds depending on mode
    // match app.mode {
    //     GuildMode => {
    //         f.render_stateful_widget(items, chunks[0], &mut app.guilds.state);
    //     }
    //     //Weird that let items isn't used, or so vscode thinks
    //     ChannelMode => {
    //         f.render_stateful_widget(items, chunks[0], &mut app.items.state);
    //     }
    // }

    // // Could be better, a lot of cloning
    // let title = app.get_current_title();
    // let chat_messages = app.get_messages();

    // //If there are messages, use those, if there aren't advertise
    // match chat_messages {
    //     Some(v) => {
    //         let chat_messages = msg_to_list(v, &right_chunks[0]);
    //         let chat =
    //             List::new(chat_messages).block(Block::default().borders(Borders::ALL).title(title));

    //         f.render_widget(chat, right_chunks[0]);
    //     }
    //     None => {
    //         let ad = vec![ListItem::new(
    //             "Check my other projects on https://github.com/DvorakDwarf",
    //         )];
    //         let chat = List::new(ad).block(Block::default().borders(Borders::ALL).title(title));

    //         f.render_widget(chat, right_chunks[0]);
    //     }
    // }

    // //The chat box is here
    // let input = Paragraph::new(cbox.input.as_ref())
    //     .style(match cbox.input_mode {
    //         InputMode::Normal => Style::default(),
    //         InputMode::Editing => Style::default().fg(Color::Yellow),
    //     })
    //     .block(Block::default().borders(Borders::ALL).title("Input"));
    // f.render_widget(input, right_chunks[1]);

    // match cbox.input_mode {
    //     InputMode::Normal => {} //hides cursor
    //     InputMode::Editing => {
    //         //Set cursor as visible and move to right spot
    //         f.set_cursor(
    //             // Put cursor past the end of the input text
    //             right_chunks[1].x + cbox.input.len() as u16 + 1,
    //             // Move one line down, from the border to the input line
    //             right_chunks[1].y + 1,
    //         )
    //     }
    // }
}

// // Converts channels into list items that work with the tui-rs widget
// fn channels_to_listitems(items: &Vec<Channel>) -> Vec<ListItem> {
//     let item_list: Vec<ListItem> = items
//         .iter()
//         .map(|i| {
//             let text = i.name.clone();
//             let lines = vec![Spans::from(text)];
//             ListItem::new(lines).style(Style::default().fg(Color::Black).bg(Color::White))
//         })
//         .collect();

//     return item_list;
// }

// // Converts guilds into list items that work with the tui-rs widget
// fn guilds_to_listitems(items: &Vec<Guild>) -> Vec<ListItem> {
//     let item_list: Vec<ListItem> = items
//         .iter()
//         .map(|i| {
//             let text = i.name.clone();
//             let lines = vec![Spans::from(text)];
//             ListItem::new(lines).style(Style::default().fg(Color::Black).bg(Color::White))
//         })
//         .collect();

//     return item_list;
// }

// // Converts messages into list items that work with the tui-rs widget
// fn msg_to_list(messages: Vec<Msg>, border: &Rect) -> Vec<ListItem> {
//     let mut items: Vec<ListItem> = Vec::new();

//     for msg in messages {
//         let name = &msg.user.name;
//         let content = &msg.content;
//         let combo = format!("{}: {}", name, content);
//         let new_item = ListItem::new(combo);

//         items.push(new_item);
//     }

//     let height = border.height as usize;

//     if height > items.len() {
//         return items;
//     } else {
//         //Weird length/index bs. +2 seems to work fine tho
//         let slice = items.as_slice()[items.len() - height + 3..].to_vec();
//         return slice;
//     }
// }
