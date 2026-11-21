- Investigate deeper observability.
- Improve validation. Some is being done by the validator crate on dtos but there is also some validation being done by repository functions.
- Upgrade backend deps and fix issues.
- the “no overlapping accepted leave” check runs before the transaction starts, so two overlapping requests accepted at the same moment could both get through.

- derbe currently does not show which user is logged in. 
- There is no logout button. 
- The list of employees in the page to create a new leave request shows all employees. 
- Check that employee emails must be unique.
- sempahore for tests?