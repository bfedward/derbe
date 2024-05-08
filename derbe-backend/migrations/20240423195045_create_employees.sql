CREATE TABLE employees (
    id SERIAL PRIMARY KEY,
    first_name VARCHAR NOT NULL,
    last_name VARCHAR NOT NULL,
    job_title VARCHAR NOT NULL,
    division_id INTEGER NOT NULL REFERENCES divisions(id),
    employment_type VARCHAR NOT NULL CHECK (employment_type IN ('PERMANENT', 'CONTRACT')),
    location_id INTEGER NOT NULL REFERENCES locations(id),
    manager_id INTEGER REFERENCES employees(id),
    email VARCHAR NOT NULL,
    phone VARCHAR,
    CONSTRAINT unique_employee_email UNIQUE (email),
    CONSTRAINT unique_employee_phone UNIQUE (phone),
    CONSTRAINT employee_cant_manage_self check (manager_id != id)
);
