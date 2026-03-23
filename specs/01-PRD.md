# timelog — Product Requirements Document

Owner: Juan Desimoni
Status: Draft
Last updated: 2026-03-23

---

## 1. Overview

### Problem

Small teams (e.g. construction crews, field workers, small companies, people working from home) need
a simple way to track working hours.

Current situation:
- Workers track time manually or not at all
- Data is inconsistent or lost
- Supervisors lack visibility into worked hours
- Corrections are painful or informal

### Users

- All user interaction is done with the web browser
- Can be on desktop, notebook or smartphone
- App should be responsive

#### Worker
- Needs to clock in/out quickly
- Wants to see and fix their own records
- Can have fixed working hours
- Likely uses mobile
- Workers can be grouped in teams
- Can belong to a team or not

#### Supervisor
- Needs visibility over all workers and teams
- Needs to manage workers and teams and working hours
- Needs summaries for decision-making
- Likely uses desktop

#### System Admin
- Full access to the system
- Only one sysadmin for instance
- Manages supervisors and system-level configuration
- Handles exceptional cases and data corrections
- Internal use only
- Likely uses desktop

---

## 2. Goals

- Allow workers to register work sessions in < 10 seconds
- Allow workers to view and edit their own time entries
- Allow supervisors to view all worker activity
- Allow supervisors to edit workers attributes like working hours or request a password reset
- Provide simple summaries of worked hours per worker
- Allow easy and intuitive usage
- Allow easy administration

---

## 3. Non-Goals (Out of Scope)

- Payroll integration
- Notifications (email/push)
- Advanced analytics
- Multi-company / multi-tenant support
- Offline-first support

---

## 4. Success Metrics

- % of workers actively logging time daily
- Average time to create a time entry
- Number of manual corrections needed per week
- Supervisor weekly usage

---

## 5. Assumptions

- Users prefer simplicity over feature richness
- Mobile-first usage for workers
- Desktop usage for supervisors and for the sysadmin
- Internet connectivity is generally available

---

## 6. Core Features

### 6.1 Worker

#### Clock in / Clock out
- Start a work session
- End a work session

#### View entries
- List past work sessions

#### Edit entries
- Modify start/end times

---

### 6.2 Supervisor

#### Manage workers
- Create worker
- Modify worker attributes

#### View entries
- View all workers' sessions
- Filter by worker / date

#### Summaries
- Total hours per worker (time range)

### 6.3 System Admin

#### Manage users
- Create, modify and delete users

#### Global visibility
- Access all data across the system

#### System operations
- Perform manual corrections
- Access directly to the DB

---

## 7. Open Questions

- Should edits be audited?
- Do we need approval flow for edits?
- How important is real-time vs eventual consistency?
- Do workers need roles beyond basic?

---

## 8. Future Considerations

- Payroll integration
- Notifications
