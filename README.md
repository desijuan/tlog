# tlog

A simple time-tracking web app for small teams. Workers clock in and out from their phone or browser; supervisors get visibility over worked hours across the team.

> Work in progress...

## Stack

- **Backend**: Rust + Axum
- **Database**: SQLite
- **Frontend**: Plain HTML, CSS, vanilla JS — no frameworks, no build step

## Features

- Workers clock in/out in seconds
- Workers can view and edit their own sessions
- Supervisors manage workers, schedules, and view session history
- Worked hours summaries by worker and date range
- Role-based access: worker, supervisor, sysadmin
- Session edit audit log

## Getting Started

```bash
cd tlog
cargo run
```

Requires Rust stable. The SQLite database file is created automatically on first run.

## Project Structure - WIP

```
tlog
├── README.md
├── schema.sql
└── specs
    ├── 01-PRD.md
    ├── 02-domain-model.md
    ├── 03-system-design.md
    ├── 04-screens-and-flows.md
    └── 05-endpoints.md
```

## License

MIT
