-- sql/schema.sql
-- Calendar TUI Database Schema
-- Uses Google Calendar IDs as universal identifiers

BEGIN TRANSACTION;

-- Main events table
CREATE TABLE IF NOT EXISTS events (
  local_id INTEGER PRIMARY KEY AUTOINCREMENT,
  event_id TEXT NOT NULL,           -- Event ID within calendar
  calendar_id TEXT NOT NULL,       -- Google Calendar ID (e.g., 'primary', 'work@company.com')
  source_type TEXT NOT NULL,           -- 'gcal' or 'orgmode' (tracks origin)
  title TEXT NOT NULL,
  description TEXT,
  location TEXT,
  start_time INTEGER NOT NULL,         -- Unix timestamp
  end_time INTEGER NOT NULL,           -- Unix timestamp
  created_at INTEGER DEFAULT (unixepoch()),
  modified_at INTEGER DEFAULT (unixepoch()),
  deleted BOOLEAN DEFAULT FALSE        -- Soft delete for sync safety

  -- Unique constraint includes calendar_id since event_id is only unique within calendar
  UNIQUE(event_id, calendar_id)
);

-- Add calendar metadata table for better UX
CREATE TABLE IF NOT EXISTS calendars (
    calendar_id TEXT PRIMARY KEY,    -- Google Calendar ID
    display_name TEXT NOT NULL,      -- Human-readable name
    color TEXT,                      -- Calendar color (hex)
    access_role TEXT,                -- owner/writer/reader
    sync_enabled BOOLEAN DEFAULT TRUE,
    last_sync_time INTEGER DEFAULT (unixepoch()),
    etag TEXT                        -- For incremental sync
);

-- Sync metadata table to track sync state per source
CREATE TABLE IF NOT EXISTS sync_metadata (
  local_id INTEGER PRIMARY KEY,
  source_type TEXT NOT NULL,

  -- Google Calendar specific
  gcal_etag TEXT,                      -- For change detection
  gcal_synced BOOLEAN DEFAULT FALSE,   -- Whether this exists in Google Calendar yet

  -- Org-mode specific  
  org_file_path TEXT,                  -- Which .org file this came from
  org_line_start INTEGER,              -- Line number in org file
  org_line_end INTEGER,                -- End line number
  org_content_hash TEXT,               -- Hash of org content for change detection

  -- For org-mode events: which calendar should they sync to?
  target_calendar_id TEXT,         -- Where to create this event in Google Calendar

  -- General sync tracking
  last_sync_time INTEGER DEFAULT (unixepoch()),
  needs_upload BOOLEAN DEFAULT FALSE,   -- Changed locally, needs sync to external
  sync_conflict BOOLEAN DEFAULT FALSE,  -- Conflict detected during sync

  FOREIGN KEY (local_id) REFERENCES events(local_id) ON DELETE CASCADE
  FOREIGN KEY (target_calendar_id) REFERENCES calendars(calendar_id)
);

-- Indexes
CREATE INDEX IF NOT EXISTS idx_events_gcal_calendar 
    ON events(event_id, calendar_id);

CREATE INDEX IF NOT EXISTS idx_events_calendar_id 
    ON events(calendar_id);

CREATE INDEX IF NOT EXISTS idx_events_source_type 
    ON events(source_type);

CREATE INDEX IF NOT EXISTS idx_events_time_range 
    ON events(start_time, end_time);

CREATE INDEX IF NOT EXISTS idx_sync_needs_upload 
    ON sync_metadata(needs_upload) WHERE needs_upload = TRUE;

CREATE INDEX IF NOT EXISTS idx_sync_target_calendar 
    ON sync_metadata(target_calendar_id);

-- Auto-update modified_at trigger (unchanged)
CREATE TRIGGER IF NOT EXISTS update_events_modified_at 
    AFTER UPDATE ON events
    FOR EACH ROW
    WHEN NEW.modified_at = OLD.modified_at
BEGIN
    UPDATE events SET modified_at = unixepoch() WHERE local_id = NEW.local_id;
END;

COMMIT;
