use std::path::Path;

use rusqlite::{Connection, Error};

use crate::{google_calendar_api::GoogleCalendarAPI};

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

    fn sync_gcal(&mut self, gcal_api: GoogleCalendarAPI) {
        
    }
}
