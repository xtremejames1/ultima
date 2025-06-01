use std::path::Path;

use chrono::Local;
use rusqlite::{params, Connection, Error};

use crate::{event::{AccessRole, CalendarEvent, GcalCalendar}, google_calendar_api::GoogleCalendarAPI};

pub struct Database {
    db: Connection
    //maybe have a vecdeque in case we need to store more in memory?
    //or to store so we don't have as many db writes?
}

impl Database {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let db = Connection::open(path)?;

        // STEP 1: Configure database settings (individual execute calls)
        db.execute("PRAGMA foreign_keys = ON", [])?;           // Data integrity
        db.execute("PRAGMA journal_mode = WAL", [])?;          // Can't change in transaction!
        db.execute("PRAGMA synchronous = NORMAL", [])?;        // Speed/safety balance
        db.execute("PRAGMA cache_size = -64000", [])?;         // 64MB cache
        db.execute("PRAGMA temp_store = MEMORY", [])?;         // Fast temp operations
        db.execute_batch(include_str!("../sql/schema.sql"))?;
        Ok(Self {
            db
        })
    }

    pub fn sync_calendar(&mut self, calendar: &mut GcalCalendar) -> Result<(), rusqlite::Error> {
        let tx = self.db.transaction()?;

        if tx.query_row(
            "SELECT COUNT(1) FROM calendars WHERE calendar_id = ?1",
            params![&calendar.id],
            |row| Ok(row.get(0) == Ok(1)))? {
            //TODO just accept remote changes tbh
            if let Some(color) = &calendar.color {
                tx.execute("UPDATE calendars SET display_color = ?1 WHERE calendar_id = ?2", params![color, &calendar.id])?;
            }
            tx.execute("UPDATE calendars SET display_name = ?1 WHERE calendar_id = ?2", params![&calendar.name, &calendar.id])?;
            let access = match &calendar.access {
                AccessRole::Owner => { "owner" }
                AccessRole::Reader => { "reader" }
                AccessRole::Writer => { "writer" }
                AccessRole::FreeBusyReader => { "freeBusyReader" }
            };
            tx.execute("UPDATE calendars SET access_role = ?1 WHERE calendar_id = ?2", params![access, &calendar.id])?;
            tx.execute("UPDATE calendars SET sync_enabled = ?1 WHERE calendar_id = ?2", params![&calendar.sync_enabled, &calendar.id])?;
            let last_sync_time = Local::now().timestamp();
            tx.execute("UPDATE calendars SET last_sync_time = ?1 WHERE calendar_id = ?2", params![last_sync_time, &calendar.id])?;
            if let Some(etag) = &calendar.etag {
                tx.execute("UPDATE calendars SET etag = ?1 WHERE calendar_id = ?2", params![etag, &calendar.id])?;
            }
        }

        if let Some(color) = &calendar.color {
            tx.execute("INSERT INTO calendars SET display_color = ?1 WHERE calendar_id = ?2", params![color, &calendar.id])?;
        }
        tx.execute("UPDATE calendars SET display_name = ?1 WHERE calendar_id = ?2", params![&calendar.name, &calendar.id])?;
        let access = match &calendar.access {
            AccessRole::Owner => { "owner" }
            AccessRole::Reader => { "reader" }
            AccessRole::Writer => { "writer" }
            AccessRole::FreeBusyReader => { "freeBusyReader" }
        };
        tx.execute("UPDATE calendars SET access_role = ?1 WHERE calendar_id = ?2", params![access, &calendar.id])?;
        tx.execute("UPDATE calendars SET sync_enabled = ?1 WHERE calendar_id = ?2", params![&calendar.sync_enabled, &calendar.id])?;
        let last_sync_time = Local::now().timestamp();
        tx.execute("UPDATE calendars SET last_sync_time = ?1 WHERE calendar_id = ?2", params![last_sync_time, &calendar.id])?;
        if let Some(etag) = &calendar.etag {
            tx.execute("UPDATE calendars SET etag = ?1 WHERE calendar_id = ?2", params![etag, &calendar.id])?;
        }

        //TODO insert into, probably a more graceful way to do this than the above
        Ok(())
    }

    pub fn sync_event(&mut self, event: &mut CalendarEvent) {
        todo!()
    }
}
