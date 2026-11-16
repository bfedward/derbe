CREATE TABLE public.leave_requests (
    id                serial                         NOT NULL,
    employee_unid     uuid                           NOT NULL,
    leave_type_id     integer                        NOT NULL,
    leave_status      public.valid_leave_statuses    NOT NULL,
    date_from         date                           NOT NULL,
    date_to           date                           NOT NULL,
    first_day_half    boolean                        DEFAULT false NOT NULL,
    last_day_half     boolean                        DEFAULT false NOT NULL,
    total_duration    double precision               NOT NULL,
    date_requested    timestamp with time zone       DEFAULT CURRENT_TIMESTAMP NOT NULL,
    date_finalised    timestamp with time zone,
    notes             text,
    CONSTRAINT leave_requests_pkey PRIMARY KEY (id),
    CONSTRAINT date_check CHECK (date_to >= date_from),
    CONSTRAINT fk_leave_requests_employee_unid FOREIGN KEY (employee_unid) REFERENCES public.employees(unid),
    CONSTRAINT fk_leave_requests_leave_type_id FOREIGN KEY (leave_type_id) REFERENCES public.leave_types(id)
);
