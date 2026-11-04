export const derbeKeys = {
  all: ["derbe"] as const,
  employees: () => [...derbeKeys.all, "employees"] as const,
  employee: (id: number) => [...derbeKeys.employees(), id] as const,
  employeeDetails: () => [...derbeKeys.all, "employee-details"] as const,
  divisions: () => [...derbeKeys.all, "divisions"] as const,
  locations: () => [...derbeKeys.all, "locations"] as const,
  leaveTypes: () => [...derbeKeys.all, "leave-types"] as const,
  leaveType: (id: number) => [...derbeKeys.leaveTypes(), id] as const,
  myLeaveAll: () => [...derbeKeys.all, "my-leave"] as const,
  myLeave: (year: number) => [...derbeKeys.all, "my-leave", year] as const,
  allLeave: (year: number, month: number) =>
    [...derbeKeys.all, "all-leave", year, month] as const,
  leaveRequest: (id: number) => [...derbeKeys.all, "leave-request", id] as const,
};
