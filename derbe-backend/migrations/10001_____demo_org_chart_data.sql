-- Local demo org chart accounts. Password for each: password.
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
    seed.first_name,
    seed.last_name,
    seed.job_title,
    seed.email,
    '$2y$10$4OohdVIAfsgqV3gooc3MRuwpzUS9srljrEr16immEyqaQVWckoi5C',
    'Manager',
    division.id,
    'Permanent',
    location.id
FROM (
    VALUES
        ('Jordan', 'Lee', 'Engineering Manager', 'jordan.lee@example.com'),
        ('Casey', 'Patel', 'People Manager', 'casey.patel@example.com'),
        ('Morgan', 'Reed', 'Operations Manager', 'morgan.reed@example.com')
) AS seed(first_name, last_name, job_title, email)
CROSS JOIN public.divisions AS division
CROSS JOIN public.locations AS location
WHERE division.name = 'Demo Division'
  AND location.name = 'Demo Location'
  AND NOT EXISTS (
      SELECT 1
      FROM public.employees
      WHERE email = seed.email
  );

INSERT INTO public.employees (
    first_name,
    last_name,
    job_title,
    email,
    password,
    role,
    division_id,
    employment_type,
    location_id,
    manager_unid
)
SELECT
    seed.first_name,
    seed.last_name,
    seed.job_title,
    seed.email,
    '$2y$10$4OohdVIAfsgqV3gooc3MRuwpzUS9srljrEr16immEyqaQVWckoi5C',
    'RegularUser',
    division.id,
    'Permanent',
    location.id,
    manager.unid
FROM (
    VALUES
        ('Taylor', 'Kim', 'Software Engineer', 'taylor.kim@example.com', 'jordan.lee@example.com'),
        ('Riley', 'Brown', 'Software Engineer', 'riley.brown@example.com', 'jordan.lee@example.com'),
        ('Avery', 'Singh', 'QA Engineer', 'avery.singh@example.com', 'jordan.lee@example.com'),
        ('Quinn', 'Garcia', 'Product Designer', 'quinn.garcia@example.com', 'jordan.lee@example.com'),
        ('Jamie', 'Chen', 'People Partner', 'jamie.chen@example.com', 'casey.patel@example.com'),
        ('Drew', 'Wilson', 'Recruiter', 'drew.wilson@example.com', 'casey.patel@example.com'),
        ('Skyler', 'Martin', 'HR Coordinator', 'skyler.martin@example.com', 'casey.patel@example.com'),
        ('Emerson', 'Davis', 'Operations Specialist', 'emerson.davis@example.com', 'morgan.reed@example.com'),
        ('Rowan', 'Clark', 'Office Coordinator', 'rowan.clark@example.com', 'morgan.reed@example.com'),
        ('Cameron', 'Lopez', 'Support Specialist', 'cameron.lopez@example.com', 'morgan.reed@example.com')
) AS seed(first_name, last_name, job_title, email, manager_email)
CROSS JOIN public.divisions AS division
CROSS JOIN public.locations AS location
CROSS JOIN public.employees AS manager
WHERE division.name = 'Demo Division'
  AND location.name = 'Demo Location'
  AND manager.email = seed.manager_email
  AND NOT EXISTS (
      SELECT 1
      FROM public.employees
      WHERE email = seed.email
  );
