CREATE TABLE public.divisions (
    id      serial               NOT NULL,
    name    character varying    NOT NULL,
    CONSTRAINT divisions_pkey PRIMARY KEY (id),
    CONSTRAINT unique_division_name UNIQUE (name)
);
