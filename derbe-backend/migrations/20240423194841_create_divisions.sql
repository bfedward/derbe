CREATE TABLE divisions (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    CONSTRAINT unique_division_name UNIQUE (name)
);