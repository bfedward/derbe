CREATE TABLE leave_requests (
    id SERIAL PRIMARY KEY,
    employee_id INTEGER NOT NULL REFERENCES employees(id),
    leave_type_id INTEGER NOT NULL REFERENCES leave_types(id),
    leave_status VALID_LEAVE_STATUSES NOT NULL,
    date_from DATE NOT NULL,
    date_to DATE NOT NULL,
    first_day_half BOOLEAN NOT NULL DEFAULT FALSE,
    last_day_half BOOLEAN NOT NULL DEFAULT FALSE,
    total_duration FLOAT NOT NULL,
    date_requested TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    date_finalised TIMESTAMPTZ,
    notes TEXT
);