-- Must be set on every connection; not persisted in the DB file.
-- PRAGMA foreign_keys = ON

-- -----------------------------------------------------------------------------
-- Base entity - user
-- -----------------------------------------------------------------------------

CREATE TABLE IF NOT EXISTS users (
    id         INTEGER PRIMARY KEY,
    name       TEXT    NOT NULL UNIQUE,
    email      TEXT    NOT NULL UNIQUE,
    pwd_hash   TEXT    NOT NULL,
    created_at INTEGER NOT NULL
);

-- -----------------------------------------------------------------------------
-- Working schedules - workers can have working schedules
-- -----------------------------------------------------------------------------

CREATE TABLE IF NOT EXISTS working_schedules (
    id         INTEGER PRIMARY KEY,
    name       TEXT    NOT NULL UNIQUE,
    sun        TEXT,
    mon        TEXT,
    tue        TEXT,
    wed        TEXT,
    thu        TEXT,
    fri        TEXT,
    sat        TEXT,
    descr      TEXT,
    created_at INTEGER NOT NULL,
    is_active  INTEGER NOT NULL DEFAULT 1
    -- Each day column stores a time range string (e.g. "08:00-12:00+13:00-17:00")
    -- or NULL (meaning the worker is not expected to work that day).
);

-- -----------------------------------------------------------------------------
-- Interfaces - worker and supervisor
-- -----------------------------------------------------------------------------

CREATE TABLE IF NOT EXISTS workers (
    user_id    INTEGER PRIMARY KEY,
    ws_id      INTEGER,
    created_at INTEGER NOT NULL,
    is_active  INTEGER NOT NULL DEFAULT 1,
    FOREIGN KEY (user_id) REFERENCES users(id)             ON DELETE CASCADE,
    FOREIGN KEY (ws_id)   REFERENCES working_schedules(id) ON DELETE SET NULL
);

CREATE TABLE IF NOT EXISTS supervisors (
    user_id    INTEGER PRIMARY KEY,
    created_at INTEGER NOT NULL,
    is_active  INTEGER NOT NULL DEFAULT 1,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- -----------------------------------------------------------------------------
-- Work sessions
-- -----------------------------------------------------------------------------

CREATE TABLE IF NOT EXISTS work_sessions (
    id         INTEGER PRIMARY KEY,
    worker_id  INTEGER NOT NULL,
    start_time INTEGER NOT NULL,
    end_time   INTEGER,           -- NULL means session is still open
    created_at INTEGER NOT NULL,
    updated_at INTEGER,           -- NULL means never updated
    FOREIGN KEY (worker_id) REFERENCES workers(user_id) ON DELETE RESTRICT
);

-- A worker may have at most one open session at a time.
-- Partial index: only rows where end_time IS NULL are covered.
CREATE UNIQUE INDEX IF NOT EXISTS one_active_session_per_worker
    ON work_sessions (worker_id)
    WHERE end_time IS NULL;

-- Enforce start_time < end_time when end_time is provided on insert.
CREATE TRIGGER IF NOT EXISTS enforce_session_time_order_insert
BEFORE INSERT ON work_sessions
WHEN NEW.end_time IS NOT NULL AND NEW.end_time <= NEW.start_time
BEGIN
    SELECT RAISE(ABORT, 'end_time must be greater than start_time');
END;

-- Enforce start_time < end_time when end_time is provided on update.
CREATE TRIGGER IF NOT EXISTS enforce_session_time_order_update
BEFORE UPDATE ON work_sessions
WHEN NEW.end_time IS NOT NULL AND NEW.end_time <= NEW.start_time
BEGIN
    SELECT RAISE(ABORT, 'end_time must be greater than start_time');
END;

-- -----------------------------------------------------------------------------
-- Work session edits
-- -----------------------------------------------------------------------------

CREATE TABLE IF NOT EXISTS work_session_edits (
    id              INTEGER PRIMARY KEY,
    ws_id           INTEGER NOT NULL,  -- work_session_id
    edited_by       INTEGER NOT NULL,  -- any user (worker or supervisor)
    prev_start_time INTEGER NOT NULL,
    prev_end_time   INTEGER,
    new_start_time  INTEGER NOT NULL,
    new_end_time    INTEGER,
    note            TEXT,
    created_at      INTEGER NOT NULL,
    FOREIGN KEY (ws_id)      REFERENCES work_sessions(id) ON DELETE RESTRICT,
    FOREIGN KEY (edited_by)  REFERENCES users(id)         ON DELETE RESTRICT
);
