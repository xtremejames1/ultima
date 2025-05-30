use std::path::Path;

use rusqlite::{Connection, Error};

use crate::event::CalendarEvents;

pub struct Database {
    db: Connection
    //maybe have a vecdeque in case we need to store more in memory?
    //or to store so we don't have as many db writes?
}

impl Database {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let db = Connection::open(path)?;
        db.execute_batch(
            "BEGIN;
CREATE TABLE events(
id TEXT PRIMARY KEY,
title TEXT,
description TEXT,
location TEXT,
start TEXT,
end TEXT,
etag TEXT,
updated BOOLEAN
);
COMMIT;"
        );
        Ok(Self {
            db
        })
    }

    fn add_event(event: CalendarEvents) {
        match event {
            CalendarEvents::Gcal(gcal_event) => {
                
            },
        }
    }
}
