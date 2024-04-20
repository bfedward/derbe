CREATE TABLE leave_types (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    default_quota FLOAT,
    can_book_in_future BOOLEAN NOT NULL,
    show_in_others_days BOOLEAN NOT NULL,
    default_in_others_days BOOLEAN NOT NULL,
    show_in_approve_days BOOLEAN NOT NULL
);
