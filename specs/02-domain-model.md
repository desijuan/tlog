# tlog — Domain Model

Owner: Juan Desimoni
Last updated: 2026-03-23

---

## 1. Core Concepts

The system models time tracking for workers within a single organization.

Key concerns:
- Recording work sessions
- Managing users and roles
- Ensuring data consistency and traceability
- SQLite as the database engine (one file per organization)

---

## 2. Entities

### 2.1 User

Represents the base identity for any authenticated actor in the system.

#### Fields

- id (INTEGER PRIMARY KEY, SQLite rowid-backed)
- name (TEXT, unique)
- email (TEXT, unique)
- pwd_hash (TEXT)
- created_at (INTEGER — Unix timestamp)

#### Notes

- All roles share this base table.
- Role membership is determined by presence in the corresponding role table
  (`workers`, `supervisors`, `sysadmin`), not by a role field.
- A user can have more than one role, except for the sysadmin. For example, a user can be both a
  supervisor and a worker in another supervisor's team.
- The sysadmin is exclusive. The sysadmin user can only be sysadmin, nothing else.
- At all times there must be exactly one sysadmin
- For workers and supervisors, delete on the `users` row cascades to the role table. For sysadmin,
  delete is restricted (the sysadmin row must be removed first).

---

### 2.2 Worker

Extends User for employees who clock in and out.

#### Fields (in `workers` table)

- user_id (INTEGER PRIMARY KEY, FK → users.id, CASCADE on delete)
- working_schedule_id (FK → working_schedules.id, SET NULL on delete, nullable)
- created_at (INTEGER — Unix timestamp)
- is_active (INTEGER, default 1) — 0 means deactivated

#### Notes

- Deactivating a worker (is_active = 0) preserves all session history.
- Only workers own work sessions.

---

### 2.3 Supervisor

Extends User for supervisors who manage workers and view reports.

#### Fields (in `supervisors` table)

- user_id (INTEGER PRIMARY KEY, FK → users.id, CASCADE on delete)
- created_at (INTEGER — Unix timestamp)
- is_active (INTEGER, default 1)

#### Notes

- No session ownership.

---

### 2.4 SysAdmin

A sealed identity — not an extension of a regular user, but a distinct kind of actor that shares the
authentication infrastructure. The sysadmin cannot be combined with any other role.

#### Fields (in `sysadmin` table)

- user_id (INTEGER PRIMARY KEY, FK → users.id, RESTRICT on delete)

#### Invariants

- **Exactly one** sysadmin must exist at all times (never zero, never more than one).
  - *At most one* is enforced by a BEFORE INSERT trigger on the `sysadmin` table.
  - *At least one* is a bootstrap invariant: the sysadmin row is created at system initialization
    and cannot be deleted without first replacing it.
- **Exclusive**: a user_id that appears in `sysadmin` must not appear in `workers` or `supervisors`,
  and vice versa. Enforced by BEFORE INSERT triggers on all three role tables.
- RESTRICT on delete: the base `users` row for the sysadmin cannot be deleted while the `sysadmin` row exists.

---

### 2.5 WorkingSchedule

Represents expected working hours, broken down by day of week.

#### Fields

- id (INTEGER PRIMARY KEY)
- name (TEXT, unique)
- sunday ... saturday (TEXT, nullable) — each stores a time range string (e.g. "09:00-17:00")
  or NULL for no work that day.
- description (TEXT)
- created_at (INTEGER — Unix timestamp)
- is_active (INTEGER, default 1)

#### Notes

- Per-day columns allow different hours on different days (e.g. half-day Fridays), unlike a single start/end pair.
- NULL for a day means the worker is not expected to work that day.
- Does not encode specific calendar dates; purely a weekly template.
- Deleting a schedule sets the FK on associated workers to NULL (SET NULL).

---

### 2.6 WorkSession

Represents a unit of work performed by a worker.

#### Fields

- id (INTEGER PRIMARY KEY, SQLite rowid-backed)
- worker_id (FK → workers.user_id)
- start_time (INTEGER — Unix timestamp)
- end_time (INTEGER, nullable — Unix timestamp)
- created_at (INTEGER)
- updated_at (INTEGER)

#### Invariants

- start_time < end_time (if end_time is set)
- start_time must be in the past (or near-present)
- Only users in the workers table can own sessions
- A worker can have at most **one active session** (end_time IS NULL), enforced by a partial unique index

#### Notes

- "Clock in" → insert session with start_time, end_time = NULL
- "Clock out" → update end_time on the active session

---

### 2.7 WorkSessionEdit

Tracks modifications to work sessions for auditability.

#### Fields

- id (INTEGER PRIMARY KEY, SQLite rowid-backed)
- session_id (FK → work_sessions.id)
- edited_by (FK → users.id)
- previous_start_time (INTEGER)
- previous_end_time (INTEGER, nullable)
- new_start_time (INTEGER)
- new_end_time (INTEGER, nullable)
- note TEXT
- edited_at (INTEGER)

#### Notes

- Append-only audit log; records are never modified.
- Captures the full before/after state of a session for each edit.
- Enables future approval workflows.

---

## 3. Relationships

- User (1) → (0..1) Worker / Supervisor / SysAdmin (role membership)
- Worker (1) → (N) WorkSession
- WorkingSchedule (1) → (N) Worker
- WorkSession (1) → (N) WorkSessionEdit
- User (1) → (N) WorkSessionEdit (as editor)

---

## 4. Role Model

### Roles

- WORKER — extension role; augments a base user with clock-in/out capability and schedule assignment
- SUPERVISOR — extension role; augments a base user with management and reporting capability
- SYSADMIN — sealed identity; not an extension, but a distinct kind of actor sharing authentication infrastructure

### Role Exclusivity

Roles are mutually exclusive. A user_id may appear in exactly one role table. This is enforced by BEFORE INSERT triggers:

- Inserting into `workers` or `supervisors` checks that the user_id is not already in `sysadmin` (and vice versa).
- Inserting into `sysadmin` checks that the user_id is not already in `workers` or `supervisors`.

The distinction between extension roles and the sealed sysadmin identity is intentional: workers and supervisors are regular users with added capabilities; the sysadmin is a separate class of actor that happens to share the `users` table for authentication purposes only.

### Permissions

| Action                     | Worker | Supervisor | Sysadmin |
| -------------------------- | ------ | ---------- | -------- |
| Clock in / clock out       | Yes    | No         | Yes      |
| Edit own sessions          | Yes    | No         | Yes      |
| View own sessions          | Yes    | Yes        | Yes      |
| View all sessions          | No     | Yes        | Yes      |
| Manage workers             | No     | Yes        | Yes      |
| Manage supervisors         | No     | No         | Yes      |

### Role Implementation

Roles are implemented as separate tables that reference `users.id`, rather than a single enum column on `users`. This means:

- Role membership = presence of a row in the corresponding table
- A user with no row in any role table has no permissions
- Adding/removing an extension role = inserting/deleting a row (no schema migration needed)
- Each role table can carry role-specific fields (e.g. `created_at` on workers)

---

## 5. Aggregates

### WorkSession Aggregate

Root: WorkSession

Includes:
- WorkSession
- WorkSessionEdit

Rules:
- All edits are recorded through the WorkSession aggregate
- The single-active-session invariant is enforced at the DB level (partial unique index) and application level

---

### User Aggregate

Root: User

Includes:
- Worker / Supervisor / SysAdmin (role row)

Rules:
- Role assignment is controlled at the application layer
- Deactivating a user (is_active = 0) preserves all associated data
- The SysAdmin role is unique system-wide; the trigger prevents a second one

---

## 6. Domain Rules (Critical)

### 6.1 Single Active Session

A worker cannot have more than one open session (end_time IS NULL) at a time.

Enforced by:
- Partial unique index: `CREATE UNIQUE INDEX one_active_session_per_worker ON work_sessions(worker_id) WHERE end_time IS NULL`
- Optionally reinforced at application level before insert

---

### 6.2 Time Validity

- start_time must be in the past (or near-present)
- end_time, if set, must be ≥ start_time

---

### 6.3 Edit Traceability

- Any modification to a session must produce a WorkSessionEdit record
- Audit records are append-only

---

### 6.4 Exactly One SysAdmin, Always

- There must be exactly one sysadmin at all times — never zero, never more than one.
- *At most one* is enforced by a BEFORE INSERT trigger on the `sysadmin` table.
- *At least one* is a bootstrap invariant: the sysadmin is created at initialization and the system considers it a misconfigured state if no sysadmin exists.
- The sysadmin user row cannot be hard-deleted (RESTRICT FK); the sysadmin role row has no `is_active` flag — deactivation is not meaningful for a sealed identity.

---

### 6.5 Role Exclusivity

- A user_id can have different roles. For example, a user can be both a worker and a supervisor.
- A sysadmin user can only be sysadmin.

---

### 6.6 Worker Deactivation

- Setting is_active = 0 on a worker disables their access
- Historical session data is retained
- No hard delete of users with session history

---

## 7. Derived Data (Not Stored)

Computed at query time, not persisted:

- Total worked hours per worker (SUM of end_time - start_time)
- Sessions per day / week
- Active session (WHERE end_time IS NULL)
- Whether a worker is currently clocked in

---

## 8. Open Design Decisions

- Do session edits require supervisor approval before taking effect?
- Should editing a closed session be blocked after a time window (e.g. 7 days)?
- Can supervisors create sessions on behalf of workers?
- Timezone handling — store UTC, convert at display?
- Should deactivated workers still appear in supervisor reports?

---

## 9. Future Extensions

- Project / Task tagging per session
- Approval workflows for session edits
- Soft-delete strategy vs current is_active pattern
- Team / group structure for workers
