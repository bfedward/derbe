CREATE TABLE public.divisions (
    id      serial               NOT NULL,
    name    character varying    NOT NULL,
    CONSTRAINT divisions_pkey PRIMARY KEY (id)
);

CREATE UNIQUE INDEX unique_division_name ON public.divisions USING btree (lower((name)::text));
