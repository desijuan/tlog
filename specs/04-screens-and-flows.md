# timelog — Screens & Flows

Owner: Juan Desimoni
Last updated: 2026-03-22

---

## Overview

This document describes the screens and user flows for each role. It is the intermediate step
between the domain model and the route definitions. Routes will be derived directly from this
document.

The app is server-rendered (Rust / Axum). Pages are full HTML responses. Actions are form POSTs or,
where a full-page reload is genuinely disruptive, small fetch calls returning HTML fragments or
JSON. JavaScript is limited to form validation and widget interactions (e.g. submitting a clock-in
button without navigating away).

---

## Shared Screens

### S-1 — Login

The entry point for all users.

**Shows:**
- Email and password fields
- Submit button

**Actions:**
- Submit credentials → on success, redirect to the role's home screen; on failure, re-render with an error message

**Notes:**
- No registration screen. Users are created by supervisors (workers) or sysadmin (supervisors).
- Session is established via a server-side cookie.

---

### S-2 — Change Password

Accessible from any role's home after login.

**Shows:**
- Current password field
- New password field
- Confirm new password field

**Actions:**
- Submit → on success, redirect to home with a confirmation message; on failure, re-render with error

---

## Worker Screens

The worker UI is mobile-first. Screens are minimal. The home screen is the primary interaction surface.

### W-1 — Worker Home

The worker's landing page after login. Designed to allow clock-in/out in under 10 seconds.

**Shows:**
- Current date and time
- If no active session: a **Clock In** button
- If an active session exists: session start time + a **Clock Out** button
- A summary row: total hours worked today

**Actions:**
- Clock In → creates a new session (start_time = now); page updates to show the active session and
  Clock Out button. This is the one case where a JS fetch + partial update is appropriate, to avoid
  a full reload disrupting the clock display.
- Clock Out → closes the active session (end_time = now); same partial update behavior.

**Notes:**
- This screen must work well on a small phone screen.
- Clock In is disabled (or hidden) if a session is already open — the DB constraint is the backstop
  but the UI should not surface it as an error.

---

### W-2 — My Sessions

A list of the worker's past and current sessions.

**Shows:**
- Sessions in reverse chronological order (most recent first)
- Per session: date, start time, end time (or "open"), duration
- Pagination or a "load more" control for long histories
- An **Edit** link per session

**Actions:**
- Edit → navigate to W-3 for that session

---

### W-3 — Edit Session

A form to correct a session's start and/or end time.

**Shows:**
- Current start time (pre-filled)
- Current end time (pre-filled, may be empty if session is open)
- Submit button
- Cancel link → back to W-2

**Actions:**
- Submit valid times → updates the session, records a WorkSessionEdit, redirects to W-2 with a success message
- Submit invalid times (end ≤ start, future start, etc.) → re-render form with inline validation errors

**Notes:**
- JS validates the form client-side before submission (time ordering, not-in-future), but server
  always re-validates.
- Editing an open session's start_time is allowed. Editing end_time on an open session effectively
  closes it.

---

## Supervisor Screens

Supervisor UI is desktop-oriented. More data density is acceptable.

### SV-1 — Supervisor Home

Landing page after login.

**Shows:**
- A summary table: each worker, their status today (clocked in / not clocked in), and total hours today
- Links to Workers list and Sessions view

**Actions:**
- Click a worker row → navigate to SV-4 (worker's session list)

---

### SV-2 — Workers List

A list of all workers.

**Shows:**
- Worker name, email, schedule name, active status
- An **Edit** link per worker
- A **New Worker** button

**Actions:**
- New Worker → navigate to SV-3
- Edit → navigate to SV-3 pre-filled for that worker

---

### SV-3 — Create / Edit Worker

A form to create a new worker or modify an existing one.

**Shows (create):**
- Name field
- Email field
- Password field (initial password; worker should change on first login)
- Working schedule selector (dropdown of active schedules, optional)
- Hired date field
- Submit button

**Shows (edit):**
- Same fields, pre-filled
- Active toggle (to deactivate / reactivate the worker)
- No password field — password reset is a separate action (see SV-3a)
- Submit button

**Actions:**
- Submit → create or update worker; redirect to SV-2 with success message
- Validation errors → re-render form with inline errors

---

### SV-3a — Reset Worker Password

A small confirmation page, reached from the edit form.

**Shows:**
- Worker name
- New password field
- Confirm password field
- Submit button
- Cancel link → back to SV-3

**Actions:**
- Submit → sets new password; redirect to SV-3 with success message

---

### SV-4 — Worker Sessions

Sessions for a single worker, visible to the supervisor.

**Shows:**
- Worker name and current status (clocked in / not)
- Sessions in reverse chronological order
- Per session: date, start time, end time, duration
- Date range filter (from / to), defaults to current week
- An **Edit** link per session
- Total hours for the filtered range

**Actions:**
- Apply filter → re-render with filtered results (plain form GET)
- Edit → navigate to SV-5 for that session

---

### SV-5 — Edit Session (Supervisor)

Same structure as W-3 but accessible to the supervisor for any worker's session.

**Shows:**
- Worker name (read-only context)
- Current start time (pre-filled)
- Current end time (pre-filled)
- Submit button
- Cancel link → back to SV-4

**Actions:**
- Submit → updates session, records WorkSessionEdit with edited_by = supervisor's user_id; redirect
  to SV-4

---

### SV-6 — Working Schedules

A list of all working schedules.

**Shows:**
- Schedule name, days and hours, active status
- An **Edit** link per schedule
- A **New Schedule** button

**Actions:**
- New Schedule → navigate to SV-7
- Edit → navigate to SV-7 pre-filled

---

### SV-7 — Create / Edit Working Schedule

A form to define a weekly schedule.

**Shows:**
- Name field
- Per day (Mon–Sun): a time range input (e.g. "09:00–17:00") or a "no work" checkbox
- Active toggle (edit only)
- Submit button

**Actions:**
- Submit → create or update schedule; redirect to SV-6 with success message
- Validation errors → re-render with inline errors

---

## SysAdmin Screens

The sysadmin UI overlaps significantly with the supervisor's. The key additions are supervisor
management and global session visibility.

### SA-1 — SysAdmin Home

Landing page after login.

**Shows:**
- Link to Supervisors list
- Link to Workers list (same as SV-2, with full access)
- Link to All Sessions

---

### SA-2 — Supervisors List

**Shows:**
- Supervisor name, email, active status
- An **Edit** link per supervisor
- A **New Supervisor** button

**Actions:**
- New Supervisor → navigate to SA-3
- Edit → navigate to SA-3 pre-filled

---

### SA-3 — Create / Edit Supervisor

Same structure as SV-3 (worker form) adapted for supervisors.

**Shows (create):**
- Name, email, password fields
- Submit button

**Shows (edit):**
- Name, email fields (pre-filled)
- Active toggle
- Submit button
- Link to SA-3a (reset password)

**Actions:**
- Submit → create or update supervisor; redirect to SA-2

---

### SA-3a — Reset Supervisor Password

Same structure as SV-3a.

---

### SA-4 — All Sessions

A global session list across all workers.

**Shows:**
- Sessions in reverse chronological order
- Per session: worker name, date, start time, end time, duration
- Filters: worker (dropdown), date range
- An **Edit** link per session
- Total hours for the filtered view

**Actions:**
- Apply filter → re-render (form GET)
- Edit → navigate to SA-5

---

### SA-5 — Edit Session (SysAdmin)

Same structure as SV-5, accessible for any session in the system.

---

## Flow Diagrams

### Authentication Flow

```
GET /login
  └── POST /login
        ├── success → redirect to /{role}/home
        └── failure → re-render /login with error
```

### Worker Clock-in/out Flow

```
W-1 (Home)
  ├── [no active session] Clock In button
  │     └── POST /api/sessions/start → partial update W-1
  └── [active session]   Clock Out button
        └── POST /api/sessions/end   → partial update W-1
```

### Worker Edit Session Flow

```
W-2 (My Sessions)
  └── Edit link
        └── GET /sessions/{id}/edit → W-3
              └── POST /sessions/{id} → redirect W-2
```

### Supervisor Worker Management Flow

```
SV-2 (Workers List)
  ├── New Worker → GET /workers/new → SV-3
  │                  └── POST /workers → redirect SV-2
  └── Edit link  → GET /workers/{id}/edit → SV-3
                     ├── POST /workers/{id} → redirect SV-2
                     └── Reset password link → GET /workers/{id}/password → SV-3a
                                                 └── POST /workers/{id}/password → redirect SV-3
```

---

## Notes on JS Usage

The only cases where JavaScript is used beyond basic form validation are:

- **Clock In / Clock Out buttons on W-1**: A fetch call prevents a full page reload from disrupting
  the live clock display. The server returns an HTML fragment that replaces the button area.
- **Client-side time validation on session edit forms** (W-3, SV-5, SA-5): Checks that end > start
  and that start is not in the future, before submission. The server always re-validates.
- **Date range pickers** (SV-4, SA-4): If the browser's native `<input type="date">` is
  insufficient, a minimal JS widget. Native inputs are preferred.

Everything else — navigation, form submission, pagination — is plain HTML.
