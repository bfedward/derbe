CREATE TABLE leave_types (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    default_quota FLOAT,
    colour VARCHAR NOT NULL DEFAULT '#FFFFFF',
    can_book_in_future BOOLEAN NOT NULL,
    show_in_others_days BOOLEAN NOT NULL,
    default_in_others_days BOOLEAN NOT NULL,
    default_in_leave_request BOOLEAN NOT NULL,
    show_in_approve_days BOOLEAN NOT NULL,
    CONSTRAINT unique_leave_type_name UNIQUE (name)
);
