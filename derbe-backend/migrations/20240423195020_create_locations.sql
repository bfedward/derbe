CREATE TABLE locations (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    CONSTRAINT unique_location_name UNIQUE (name)
);