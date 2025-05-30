use chrono::{DateTime, Utc};
use google_calendar3::api::Event;

/// CalendarEvent is designed for events rendering via the TUI
pub trait CalendarEvent {
    fn get_title(&self) -> String;
    fn get_description(&self) -> Option<String>;
    fn get_location(&self) -> Option<String>;
    fn get_start_time(&self) -> DateTime<Utc>;
    fn get_end_time(&self) -> DateTime<Utc>;
    fn set_title(&mut self, title: String);
    fn set_description(&mut self, description: String);
    fn set_location(&mut self, location: String);
    fn set_start_time(&mut self, start_time: DateTime<Utc>);
    fn set_end_time(&mut self, end_time: DateTime<Utc>);
}

pub enum CalendarEvents {
    Gcal(GcalEvent)
}

pub struct GcalEvent {
    title: String,
    description: Option<String>,
    location: Option<String>,
    start_time: DateTime<Utc>,
    end_time: DateTime<Utc>,
    etag: String,
    id: String,
    updated: bool, // Updated locally since last sync
}

impl GcalEvent {
    pub fn new(event: Event) -> Result<Self, ()> {
        let title = event.summary.expect("No title provided");
        let description = event.description;
        let location = event.location;
        let start_time = event.start.expect("No start time provided").date_time.expect("Unable to convert given start time");
        let end_time = event.end.expect("No end time provided").date_time.expect("Unable to convert given end time");
        let etag = event.etag.expect("No etag provided");
        let id = event.id.expect("No id provided");
        let updated = false;

        Ok(Self {
            title,
            description,
            location,
            start_time,
            end_time,
            etag,
            id,
            updated,
        })
    }

    pub fn update(event: Event) {
        todo!()
    }
    
    //TODO make it so that updated is set to false after syncing

    pub fn get_etag(&self) -> String {
        self.etag.clone()
    }
}

impl CalendarEvent for GcalEvent {
    fn get_title(&self) -> String {
        self.title.clone()
    }

    fn get_description(&self) -> Option<String> {
        self.description.clone()
    }

    fn get_location(&self) -> Option<String> {
        self.location.clone()
    }

    fn get_start_time(&self) -> DateTime<Utc> {
        self.start_time
    }

    fn get_end_time(&self) -> DateTime<Utc> {
        self.end_time
    }

    fn set_title(&mut self, title: String) {
        self.title = title;
        self.updated = true;
    }

    fn set_description(&mut self, description: String) {
        self.description = Some(description);
        self.updated = true;
    }
    
    fn set_location(&mut self, location: String) {
        self.location = Some(location);
        self.updated = true;
    }

    fn set_start_time(&mut self, start_time: DateTime<Utc>) {
        self.start_time = start_time;
        self.updated = true;
    }

    fn set_end_time(&mut self, end_time: DateTime<Utc>) {
        self.end_time = end_time;
        self.updated = true;
    }
}
