# TSAHEYLU: Timer Service Context

## Overview
**TSAHEYLU** (The Bond) is the time tracking spiritual node of the HomeManager ecosystem. It allows family members to track time spent on various projects and tasks, creating a bond between their actions and the flow of time.

## Responsibilities
- **Time Entries**: Recording start/end times, durations, and descriptions.
- **Unified Toggle**: Managing the active timer state (Pomodoro or manual flow).
- **Integration**: Linking time entries to **EYWA'S HEART** (Projects/Tags).

## Naming Origin
Named **TSAHEYLU** because time tracking represents the neural connection (bond) between the user and their work/activity, just as the Na'vi make Tsaheylu with Pandora's creatures.

## Schema
This service owns the `tsaheylu` schema in the `eywa_db` database.

## Technical Details
- **Framework**: Axum (via `eywa-axum`)
- **Database**: PostgreSQL (managed via Sea-ORM)
- **Migrations**: Found in the `migration/` directory.
- **Port**: 3004
