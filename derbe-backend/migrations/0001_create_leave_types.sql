CREATE TYPE public.valid_others_days_visibilities AS ENUM (
    'Hidden',
    'Shown',
    'ShownByDefault'
);

CREATE TABLE public.leave_types (
    id                          serial                                 NOT NULL,
    name                        character varying                      NOT NULL,
    default_quota               double precision,
    colour                      character varying                      DEFAULT '#FFFFFF'::character varying NOT NULL,
    can_book_in_future          boolean                                NOT NULL,
    others_days_visibility      public.valid_others_days_visibilities  NOT NULL,
    default_in_leave_request    boolean                                NOT NULL,
    show_in_approve_days        boolean                                NOT NULL,
    CONSTRAINT leave_types_pkey PRIMARY KEY (id),
    CONSTRAINT unique_leave_type_name UNIQUE (name)
);
