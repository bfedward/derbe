AUTHENTICATION:
-- Tower for middleware.
-- Add login endpoint for anybody to call. Endpoint responds with a JSON token with expiry time (30 mins??) and the user's role. 
-- Frontend needs to perform token refresh when a token expires.
-- Whole app is protected by default. Cannot call any endpoint without a bearer token in the headers.
-- FOR NOW: set up seed data with one user of each type. bcrypt their passwords.
-- Login screen with email and password. Redirect the user to the login screen if they don't have a bearer token.
-- WATCH THIS!!!!! https://rustify.rs/ai/lessons/fullstack/W5/D23-Auth/Lecture
-- Look at full-stack repo W5D23 for examples.

- Access control: roles - employee, manager, admin
-- Employee: can create their own leave requests, see All Leave, see Organisation, and see configuration screens but read-only.
-- Manager: can do everything an Employee can, but can also create employees approve leave requests.
-- Admin: can do everythign a manager can but can also create/modify config items.

-- User Roles screen (accessible to Admin only)
-- shows a grid of all users, their role and their permission
-- New database table: user_roles. 
-- Enum of Roles (Admin, Manager, Employee). Use strum with the enum. Enforced with constraint in posgres - avoids making another migration to change the enum in posgres. 
-- Add constraints in postgres to ensure that the Role enum matches the Rust layer.
-- Implement a trait for roles to implement a type of permission. Implement a read function or write function. 
-- Employee could just have permission to create a leave request.
-- Employee struct would impl the Permission trait to only read access.
-- Access controller. Force implementation of the trait.
-- Add integration tests for permissions: authentication, authorisation. Go through the whole leave request flow: ensure a user can only read or write at each step, e.g. cannot create leave requests for another user.




- Investigate deeper observability.
- Improve validation. Some is being done by the validator crate on dtos but there is also some validation being done by repository functions.
- Upgrade backend deps and fix issues.