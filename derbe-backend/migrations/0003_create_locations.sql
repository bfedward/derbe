CREATE TABLE public.locations (
    id      serial               NOT NULL,
    name    character varying    NOT NULL,
    CONSTRAINT locations_pkey PRIMARY KEY (id),
    CONSTRAINT unique_location_name UNIQUE (name)
);
