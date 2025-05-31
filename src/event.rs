use chrono::{DateTime, Local, Utc};
use google_calendar3::api::{CalendarListEntry, Event};

//TODO make sure to add assertions that the events match calendar id
#[derive(Debug, Clone)]
pub struct GcalCalendar {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub events: Vec<CalendarEvent>,
    pub access: AccessRole,
    pub sync_enabled: bool,
    pub etag: Option<String>,
    pub last_sync_time: DateTime<Utc>,
}

impl GcalCalendar {
    pub fn from_calendar_list_entry(entry: CalendarListEntry) -> Result<Self, ()> {
        let id = entry.id.expect("Calendar ID not provided");
        let name = entry.summary.expect("Calendar name not provided");
        let description = entry.description;
        let events: Vec<CalendarEvent> = Vec::new();
        let access = match entry.access_role.expect("Calendar Access Role not provided").as_str() {
            "freeBusyReader" => {Some(AccessRole::FreeBusyReader)},
            "reader" => {Some(AccessRole::Reader)},
            "writer" => {Some(AccessRole::Writer)},
            "owner" => {Some(AccessRole::Owner)},
            _ => {None}
        }.expect("Invalid Access Role");
        let sync_enabled = true;
        let etag = entry.etag;
        let last_sync_time = Local::now().to_utc(); // TODO Still yet to sync, how to resolve?
        Ok(Self {
            id,
            name,
            description,
            events,
            access,
            sync_enabled,
            etag,
            last_sync_time
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AccessRole {
    Owner,
    Writer,
    Reader,
    FreeBusyReader,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SourceType {
    GoogleCalendar,
}

/// CalendarEvent is designed for events rendering via the TUI
// TODO we should change it so that its just get_render_event if above is the case, since we can
// just make these field public if necessary. at the end of the day, all of the events need to be
// gcal events i think so idk if this matters at all.

#[derive(Debug, Clone)]
pub struct CalendarEvent {
    pub title: String,
    pub description: Option<String>,
    pub location: Option<String>,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub etag: String,
    pub event_id: String,
    pub calendar_id: String,
    pub source_type: SourceType,
    pub updated: bool, // Updated locally since last sync
}

impl CalendarEvent {
    pub fn from_gcal_api(event: Event, calendar_id: String) -> Result<Self, ()> {
        let title = event.summary.expect("No title provided");
        let description = event.description;
        let location = event.location;
        let start_time = event.start.expect("No start time provided").date_time.expect("Unable to convert given start time");
        let end_time = event.end.expect("No end time provided").date_time.expect("Unable to convert given end time");
        let etag = event.etag.expect("No etag provided");
        let event_id = event.id.expect("No id provided");
        let source_type = SourceType::GoogleCalendar;
        let updated = false;

        Ok(Self {
            title,
            description,
            location,
            start_time,
            end_time,
            etag,
            event_id,
            calendar_id,
            source_type,
            updated,
        })
    }

    pub fn update(event: Event) {
        todo!()
    }

    //TODO make it so that updated is set to false after syncing and set to true after changing
}
