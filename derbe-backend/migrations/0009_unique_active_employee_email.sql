-- One active employee per email address. Emails are already stored in lower case
-- (employees_email_check), so this is effectively case-insensitive.
-- Soft-deleted employees don't keep their address, so it can be reused.
CREATE UNIQUE INDEX employees_email_active_key
    ON public.employees (email)
    WHERE deleted_at IS NULL;