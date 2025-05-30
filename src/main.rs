mod google_calendar_api;
mod event;
mod database;
mod tui;

use std::io::Error;
use chrono::Local;
use google_calendar_api::GoogleCalendarAPI;

#[tokio::main]
async fn main() -> Result<(), Error> {
    rustls::crypto::aws_lc_rs::default_provider().install_default().unwrap();
    // TUI:
    let gcal = GoogleCalendarAPI::new().await.unwrap();
    let result = gcal.get_event_names();
    for title in result.await.unwrap() {
        print!("{}, ", title);
    }

    Ok(())
    // let mut terminal = ratatui::init();
    // let calendar_result = tui::CalendarTextUserInterface::new(Local::now().date_naive()).run(&mut terminal);
    // ratatui::restore();
    // calendar_result
}


