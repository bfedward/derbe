-- Correct the original demo account and add a separate local admin account.
UPDATE public.employees
SET role = 'RegularUser'
WHERE email = 'employee@example.com';

INSERT INTO public.employees (
    first_name,
    last_name,
    job_title,
    email,
    password,
    role,
    division_id,
    employment_type,
    location_id
)
SELECT
    'Demo',
    'Admin',
    'Demo Admin',
    'admin@example.com',
    '$2y$10$4OohdVIAfsgqV3gooc3MRuwpzUS9srljrEr16immEyqaQVWckoi5C',
    'Admin',
    division.id,
    'Permanent',
    location.id
FROM public.divisions AS division
CROSS JOIN public.locations AS location
WHERE division.name = 'Demo Division'
  AND location.name = 'Demo Location'
  AND NOT EXISTS (
      SELECT 1
      FROM public.employees
      WHERE email = 'admin@example.com'
  );
