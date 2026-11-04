-- Accounts, teams, characters and the game log (phase 2a).

CREATE TABLE users (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL UNIQUE COLLATE NOCASE,
    password_hash TEXT NOT NULL,
    role TEXT NOT NULL CHECK (role IN ('player', 'gm')),
    created_at INTEGER NOT NULL
);

-- Login sessions; only the SHA-256 of the cookie token is stored.
CREATE TABLE sessions (
    token_hash TEXT PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    created_at INTEGER NOT NULL
);

-- `state` is the team part of the engine state as JSON (mode, rulings,
-- Story Points, pending decisions); the actors live in `characters`.
CREATE TABLE teams (
    id INTEGER PRIMARY KEY,
    gm_id INTEGER NOT NULL REFERENCES users (id),
    name TEXT NOT NULL,
    invite_token TEXT NOT NULL UNIQUE,
    state TEXT NOT NULL,
    created_at INTEGER NOT NULL
);

CREATE TABLE members (
    team_id INTEGER NOT NULL REFERENCES teams (id) ON DELETE CASCADE,
    user_id INTEGER NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    PRIMARY KEY (team_id, user_id)
);

-- `data` holds the sheet and the engine actor as JSON. A character is in
-- at most one team (A-04).
CREATE TABLE characters (
    id INTEGER PRIMARY KEY,
    owner_id INTEGER NOT NULL REFERENCES users (id),
    team_id INTEGER REFERENCES teams (id) ON DELETE SET NULL,
    name TEXT NOT NULL,
    data TEXT NOT NULL,
    created_at INTEGER NOT NULL
);

-- The shared log. `visible_to` is 'all' or 'gm'.
CREATE TABLE events (
    id INTEGER PRIMARY KEY,
    team_id INTEGER NOT NULL REFERENCES teams (id) ON DELETE CASCADE,
    user_id INTEGER REFERENCES users (id),
    visible_to TEXT NOT NULL,
    payload TEXT NOT NULL,
    created_at INTEGER NOT NULL
);
CREATE INDEX events_by_team ON events (team_id, id);

-- Team state and its characters before a command, for undo.
CREATE TABLE snapshots (
    id INTEGER PRIMARY KEY,
    team_id INTEGER NOT NULL REFERENCES teams (id) ON DELETE CASCADE,
    state TEXT NOT NULL,
    created_at INTEGER NOT NULL
);
