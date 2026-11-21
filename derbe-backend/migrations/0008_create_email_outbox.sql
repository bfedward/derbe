CREATE TABLE public.email_outbox (
    id                 bigserial                   NOT NULL,
    event_key          text                        NOT NULL,
    recipient          text                        NOT NULL,
    template           text                        NOT NULL,
    payload            jsonb                       NOT NULL,
    created_at         timestamp with time zone    DEFAULT now() NOT NULL,
    next_attempt_at    timestamp with time zone    DEFAULT now() NOT NULL,
    sent_at            timestamp with time zone,
    attempt_count      integer                     DEFAULT 0 NOT NULL,
    locked_until       timestamp with time zone,
    last_error         text,
    CONSTRAINT email_outbox_pkey PRIMARY KEY (id),
    CONSTRAINT email_outbox_event_key_key UNIQUE (event_key),
    CONSTRAINT email_outbox_attempt_count_check CHECK (attempt_count >= 0)
);

-- The worker only looks at unsent rows, ordered by when they are due.
CREATE INDEX email_outbox_pending_idx
    ON public.email_outbox (next_attempt_at, id)
    WHERE sent_at IS NULL;