CREATE TABLE Command (
    -- Auto-incrementing primary key
    id INTEGER PRIMARY KEY,
    -- Name of the command. Is unique
    name VARCHAR(255) UNIQUE NOT NULL,
    -- The actual command to execute. E.g. ls -l | wc -l
    statement TEXT NOT NULL,
    -- The description of the command to be used in the help command.
    -- The more descriptive the better.
    description TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
