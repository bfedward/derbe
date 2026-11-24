-- Local demo account for signing in after a fresh database setup.
-- Password: password
INSERT INTO public.divisions (name)
VALUES ('Demo Division')
ON CONFLICT (lower(name)) DO NOTHING;

INSERT INTO public.locations (name)
VALUES ('Demo Location')
ON CONFLICT (lower(name)) DO NOTHING;

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
    'Employee',
    'Demo Account',
    'employee@example.com',
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
      WHERE email = 'employee@example.com'
  );
