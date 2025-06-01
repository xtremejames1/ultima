use crate::{database::Database, google_calendar_api::GoogleCalendarAPI};

pub struct ApplicationState {
    gcal_api: GoogleCalendarAPI,
    db: Database,
}

impl ApplicationState {
    pub fn new(gcal_api: GoogleCalendarAPI, db: Database) -> Self {
        Self {
            gcal_api,
            db,
        }
    }

    /// Import all Google Calendar events into the database
    // TODO, make it possible to select which calendars to add before adding
    pub async fn init_gcal(&mut self) -> Result<(), ()> {
        let mut calendars = self.gcal_api.get_calendars().await;
        for mut calendar in calendars.expect("Unable to retrieve calendars") {
            self.db.sync_calendar(&mut calendar);
            for mut event in calendar.events {
                self.db.sync_event(&mut event);
            }
        }
        Ok(())
    }
}
