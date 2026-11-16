CREATE TABLE public.locations (
    id      serial               NOT NULL,
    name    character varying    NOT NULL,
    CONSTRAINT locations_pkey PRIMARY KEY (id)
);

CREATE UNIQUE INDEX unique_location_name ON public.locations USING btree (lower((name)::text));
