import type { EmployeeRole } from "@/bindings/EmployeeRole";
import type { EmploymentType } from "@/bindings/EmploymentType";

export const NONE_OPTION = "none";

export const EMPLOYMENT_TYPES = [
  "Permanent",
  "Contract",
] as const satisfies readonly EmploymentType[];

export const ROLE_LABELS: Record<EmployeeRole, string> = {
  Admin: "Admin",
  Manager: "Manager",
  RegularUser: "Employee",
  Demo: "Demo",
};

/** Roles an admin may give a new employee. A manager may only give RegularUser. */
export const CREATABLE_ROLES = [
  "Manager",
  "RegularUser",
] as const satisfies readonly EmployeeRole[];

/** Roles that can be someone's manager. */
export const MANAGER_ROLES = [
  "Admin",
  "Manager",
] as const satisfies readonly EmployeeRole[];
