-- Add migration script here
CREATE TABLE Command {
    -- Auto-incrementing primary key
    id INTEGER PRIMARY KEY,
    -- Name of the command. Is unique
    name VARCHAR(255) UNIQUE NOT NULL,
    -- Alias which can be used to execute the command
    alias VARCHAR(16) UNIQUE NOT NULL,
    -- The actual command to execute. E.g. ls -l | wc -l
    command TEXT NOT NULL,
    -- The description of the command to be used in the help command.
    -- The more descriptiove the better.
    description TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
}
