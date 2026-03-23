# tlog — System Design

Owner: Juan Desimoni
Last updated: 2026-03-22

---

## Single-Tenancy Strategy

The system is single-tenant per deployment.

Each organization is backed by a separate SQLite database file.

Implications:
- Strong data isolation between organizations
- Simple backup and restore (file copy is sufficient)
- No cross-organization queries
- Schema migrations run per-file

---

## Database Configuration

Foreign key enforcement must be enabled at connection time (SQLite does not enable it by default):

```sql
PRAGMA foreign_keys = ON;
```

This must be set on every connection. It is not persisted in the database file.

---

## Role Table Pattern

Rather than storing role as an enum column on `users`, roles are modeled as separate tables:

```
users           — base identity (name, email, pwd_hash)
├── workers     — extension role: user_id FK, hired_at, working_schedule_id, is_active
├── supervisors — extension role: user_id FK, is_active
└── sysadmin    — sealed identity: user_id FK (exactly one row, always)
```

All role tables use `user_id` as the primary key.

**Workers and supervisors** are *extension roles* — they augment a base user with extra fields and permissions. They can be activated, deactivated, and their fields can evolve independently.

**SysAdmin** is a *sealed identity* — it is not an extension of a regular user but a distinct kind of actor that shares the `users` table only for authentication. It carries no extra fields, cannot be combined with any other role, and has no `is_active` flag (it is always active by definition).

**Why this approach:**
- Each role can carry its own fields without nulling out irrelevant columns on `users`
- Role membership is explicit: row present = role held
- Adding a new extension role requires a new table, not a schema change to `users`

**Trade-off:** resolving a user's role requires checking up to three tables. For this system's scale, this is acceptable.

---

## Key Constraints

### Single Active Session Per Worker

A worker may have at most one open session (end_time IS NULL) at any time:

```sql
CREATE UNIQUE INDEX one_active_session_per_worker
ON work_sessions(worker_id)
WHERE end_time IS NULL;
```

This is a partial unique index — it only applies to rows where `end_time IS NULL`, so closed sessions are unconstrained.

### Exactly One SysAdmin

Exactly one sysadmin must exist in the system at all times.

*At most one* is enforced by a BEFORE INSERT trigger:

```sql
CREATE TRIGGER enforce_single_sysadmin
BEFORE INSERT ON sysadmin
WHEN (SELECT COUNT(*) FROM sysadmin) >= 1
BEGIN
    SELECT RAISE(ABORT, 'Only one sysadmin allowed');
END;
```

*At least one* is a bootstrap invariant enforced at initialization time, not at the DB layer.

### Role Exclusivity

A user_id may appear in exactly one role table. This is enforced by BEFORE INSERT triggers on all three role tables. For example, inserting into `sysadmin` must fail if the user_id already exists in `workers` or `supervisors`, and vice versa:

```sql
CREATE TRIGGER enforce_sysadmin_exclusivity
BEFORE INSERT ON sysadmin
BEGIN
    SELECT RAISE(ABORT, 'User already has a role')
    WHERE EXISTS (SELECT 1 FROM workers WHERE user_id = NEW.user_id)
       OR EXISTS (SELECT 1 FROM supervisors WHERE user_id = NEW.user_id);
END;
```

Symmetric triggers are required on `workers` and `supervisors` to check against `sysadmin` (and against each other).

---

## Timestamp Convention

All timestamps are stored as INTEGER (Unix epoch, seconds). Timezone conversion is handled at the application layer. The canonical storage timezone is UTC.

---

## Soft Deactivation

Users are never hard-deleted while they have associated data. Instead:

- Workers and supervisors have an `is_active` column (1 = active, 0 = deactivated)
- Deactivation preserves session history and audit records
- The `sysadmin` table uses `ON DELETE RESTRICT` to prevent accidental removal of the admin user

---

## Future Considerations

- Migration tooling per SQLite file when schema evolves
- Read-only replica or export mechanism for reporting
