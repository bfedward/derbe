- Investigate deeper observability.
- Improve validation. Some is being done by the validator crate on dtos but there is also some validation being done by repository functions.
- Upgrade backend deps and fix issues.
- the “no overlapping accepted leave” check runs before the transaction starts, so two overlapping requests accepted at the same moment could both get through.
- The list of employees in the page to create a new leave request shows all employees. 
- sempahore for tests?
- make employees verify their email address before sending emails to them??
- Create an employee -> send an email for them to set their password??
- "Forgot my password" link on login screen -> sends email for password reset??
- Frontend: hide buttons and links depending on the current user's role. E.g. don't let a regular user create employees.
- Remove the suggested values appearing for all form fields in the browser.
- When creating a leave request, a regular employee should only see themselves in the employee dropdown (or don't show the dropdown at all?). A manager can only see themselves and their team employees. An admin can see everybody.
- Don't show the "More Info Required", "Reject" and "Accept" butt
- a manager can move a report to another team.
- a regular user can't open their own details, but they can save changes to them through the API, including their own manager_unid, division_id and email.


- Employee could be promoted to manager?