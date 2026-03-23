# tlog — Endpoints

Owner: Juan Desimoni
Last updated: 2026-03-23

---

## Architecture Overview

### Stack

- **Backend**: Rust + Axum
- **Database**: SQLite (one file per organization)
- **Auth**: JWT in HttpOnly Cookie
- **Frontend**: Plain HTML + CSS + vanilla JS (no libraries, no frameworks)

### Route Families

**`/app/...`** — serves static HTML files from disk. Every handler verifies the JWT cookie before
serving. If the cookie is missing or invalid, redirects to `/app/login`. Files are served through
explicit Axum handlers (not `ServeDir`) so that auth can be enforced and caching can be added later.
CSS and JS assets are also served through handlers, not exposed directly.

**`/api/...`** — returns JSON. Every handler verifies the JWT cookie. Returns `401` if missing or
invalid. The role of the authenticated user is resolved from the database on each request (not
stored in the JWT).

### JWT

The JWT contains only `user_id` and optionally `name`. The role is resolved by querying the
database on every authenticated request.

### Page Hydration Flow

1. Browser requests `/app/<page>` — server verifies cookie, serves HTML shell.
2. Page JS fetches `/api/...` endpoints to populate data.
3. On `401`, JS redirects to `/app/login`.

---

## Auth (all roles)

### Pages `/app/`

| Method | Path                   | Description                                             |
| ------ | -----                  | ------------                                            |
| GET    | `/app/login`           | Login form. Only public page — does not require cookie. |
| GET    | `/app/change-password` | Change password form. Requires cookie.                  |

### API `/api/`

| Method | Path                        | Description                                                                                  |
| ------ | ----                        | -----------                                                                                  |
| POST   | `/api/auth/login`           | Receives email + password. Validates credentials. Responds with `Set-Cookie` (JWT HttpOnly). |
| POST   | `/api/auth/logout`          | Clears the JWT cookie.                                                                       |
| POST   | `/api/auth/change-password` | Receives current password + new password. Requires cookie.                                   |

---

## Worker

### Pages `/app/`

| Method | Path                            | Description                                                                                |
| ------ | -----                           | ------------                                                                               |
| GET    | `/app/worker/home`              | Main worker page. Shows current clock-in status, clock in/out button, and recent sessions. |
| GET    | `/app/worker/sessions`          | Full session history with edit links.                                                      |
| GET    | `/app/worker/sessions/:id/edit` | Edit form for a specific session.                                                          |

### API `/api/`

| Method | Path                             | Description                                                                                         |
| ------ | -----                            | ------------                                                                                        |
| GET    | `/api/worker/me`                 | Returns authenticated worker's data (name, schedule, is_active).                                    |
| GET    | `/api/worker/sessions`           | Sessions for the authenticated worker. Accepts date range filters.                                  |
| GET    | `/api/worker/sessions/active`    | Returns the current open session (end_time IS NULL), or 404 if none.                                |
| POST   | `/api/worker/sessions/clock-in`  | Creates a new session with start_time = now.                                                        |
| POST   | `/api/worker/sessions/clock-out` | Closes the active session with end_time = now.                                                      |
| PATCH  | `/api/worker/sessions/:id`       | Edits start_time and/or end_time of a session (open or closed). Generates a WorkSessionEdit record. |

---

## Supervisor

### Pages `/app/`

| Method | Path                                 | Description                                                        |
| ------ | ----                                 | -----------                                                        |
| GET    | `/app/supervisor/home`               | Overview: worker list, who is currently clocked in.                |
| GET    | `/app/supervisor/workers`            | Full list of workers with attributes.                              |
| GET    | `/app/supervisor/workers/new`        | Form to create a new worker.                                       |
| GET    | `/app/supervisor/workers/:id`        | Worker detail: attributes, assigned schedule, session history.     |
| GET    | `/app/supervisor/workers/:id/edit`   | Form to edit worker attributes (name, email, schedule, is_active). |
| GET    | `/app/supervisor/sessions`           | All sessions across all workers. Supports filters.                 |
| GET    | `/app/supervisor/reports`            | Worked hours summary per worker over a date range.                 |
| GET    | `/app/supervisor/schedules`          | List of working schedules.                                         |
| GET    | `/app/supervisor/schedules/new`      | Form to create a new schedule.                                     |
| GET    | `/app/supervisor/schedules/:id/edit` | Form to edit an existing schedule.                                 |

### API `/api/`

| Method | Path                                   | Description                                                 |
| ------ | ----                                   | -----------                                                 |
| GET    | `/api/supervisor/workers`              | List of workers with current clock-in status.               |
| GET    | `/api/supervisor/workers/:id`          | Worker detail.                                              |
| POST   | `/api/supervisor/workers`              | Creates a new worker (creates user row + worker row).       |
| PATCH  | `/api/supervisor/workers/:id`          | Edits worker attributes.                                    |
| GET    | `/api/supervisor/workers/:id/sessions` | Sessions for a specific worker. Accepts date range filters. |
| GET    | `/api/supervisor/sessions`             | All sessions. Accepts filters by worker and date.           |
| GET    | `/api/supervisor/reports/hours`        | Total hours per worker over a date range.                   |
| GET    | `/api/supervisor/schedules`            | List of available schedules.                                |
| GET    | `/api/supervisor/schedules/:id`        | Detail of a specific schedule (for edit form hydration).    |
| POST   | `/api/supervisor/schedules`            | Creates a new schedule.                                     |
| PATCH  | `/api/supervisor/schedules/:id`        | Edits an existing schedule.                                 |

---

## Sysadmin

### Pages `/app/`

| Method | Path                            | Description                                  |
| ------ | ----                            | -----------                                  |
| GET    | `/app/sysadmin/home`            | System overview.                             |
| GET    | `/app/sysadmin/users`           | List of all users (workers and supervisors). |
| GET    | `/app/sysadmin/users/new`       | Form to create a user of any role.           |
| GET    | `/app/sysadmin/users/:id`       | User detail.                                 |
| GET    | `/app/sysadmin/users/:id/edit`  | Form to edit any user attribute.             |
| GET    | `/app/sysadmin/supervisors`     | List of supervisors.                         |
| GET    | `/app/sysadmin/supervisors/new` | Form to create a new supervisor.             |

### API `/api/`

| Method | Path                                     | Description                                            |
| ------ | ----                                     | -----------                                            |
| GET    | `/api/sysadmin/users`                    | All users with their roles.                            |
| GET    | `/api/sysadmin/users/:id`                | User detail.                                           |
| POST   | `/api/sysadmin/users`                    | Creates a user with the specified role.                |
| PATCH  | `/api/sysadmin/users/:id`                | Edits any attribute of a user.                         |
| DELETE | `/api/sysadmin/users/:id`                | Deletes a user (subject to domain model restrictions). |
| POST   | `/api/sysadmin/users/:id/reset-password` | Forces a password reset for a user.                    |
