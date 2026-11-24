import { useMemo } from "react";
import type { EmployeeResponse } from "@/bindings/EmployeeResponse";

export type OrganisationNode = {
  employee: EmployeeResponse;
  reports: OrganisationNode[];
};

export function useOrganisationChart(employees: EmployeeResponse[]) {
  return useMemo(() => {
    const nodes = new Map<string, OrganisationNode>();
    for (const employee of employees) {
      nodes.set(employee.employee_unid, { employee, reports: [] });
    }

    const admins = employees
      .filter((employee) => employee.role === "Admin")
      .map((employee) => nodes.get(employee.employee_unid))
      .filter((node): node is OrganisationNode => node !== undefined);
    const managers = employees
      .filter((employee) => employee.role === "Manager")
      .map((employee) => nodes.get(employee.employee_unid))
      .filter((node): node is OrganisationNode => node !== undefined);
    const staff = employees
      .filter((employee) => employee.role !== "Admin" && employee.role !== "Manager")
      .map((employee) => nodes.get(employee.employee_unid))
      .filter((node): node is OrganisationNode => node !== undefined);

    for (const manager of managers) {
      const assignedAdmin = manager.employee.manager_unid
        ? admins.find(
            (admin) => admin.employee.employee_unid === manager.employee.manager_unid,
          )
        : undefined;
      const admin = assignedAdmin ?? admins[0];
      if (admin) admin.reports.push(manager);
    }

    for (const employee of staff) {
      const assignedManager = employee.employee.manager_unid
        ? managers.find(
            (manager) =>
              manager.employee.employee_unid === employee.employee.manager_unid,
          )
        : undefined;
      const manager = assignedManager ?? managers[0];
      if (manager) manager.reports.push(employee);
      else if (admins[0]) admins[0].reports.push(employee);
    }

    return admins.length > 0 ? admins : managers.length > 0 ? managers : staff;
  }, [employees]);
}
