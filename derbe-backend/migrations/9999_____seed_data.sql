-- Local demo accounts for signing in after a fresh database setup.
-- Password: password
INSERT INTO public.divisions (name)
VALUES ('Demo Division')
ON CONFLICT (lower(name)) DO NOTHING;

INSERT INTO public.locations (name)
VALUES ('Demo Location')
ON CONFLICT (lower(name)) DO NOTHING;

UPDATE public.employees
SET role = CASE email
    WHEN 'admin@example.com' THEN 'Admin'
    WHEN 'employee@example.com' THEN 'RegularUser'
END
WHERE email IN ('admin@example.com', 'employee@example.com');

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
    demo.first_name,
    demo.last_name,
    demo.job_title,
    demo.email,
    '$2y$10$4OohdVIAfsgqV3gooc3MRuwpzUS9srljrEr16immEyqaQVWckoi5C',
    demo.role,
    division.id,
    'Permanent',
    location.id
FROM public.divisions AS division
CROSS JOIN public.locations AS location
CROSS JOIN (VALUES
    ('Demo', 'Admin', 'Demo Admin', 'admin@example.com', 'Admin'),
    ('Demo', 'Employee', 'Demo Account', 'employee@example.com', 'RegularUser')
) AS demo(first_name, last_name, job_title, email, role)
WHERE division.name = 'Demo Division'
  AND location.name = 'Demo Location'
  AND NOT EXISTS (
      SELECT 1
      FROM public.employees
      WHERE email = demo.email
  );
