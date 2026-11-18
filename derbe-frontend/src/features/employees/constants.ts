import type { EmploymentType } from "@/bindings/EmploymentType";

export const NONE_OPTION = "none";

export const EMPLOYMENT_TYPES = [
  "Permanent",
  "Contract",
] as const satisfies readonly EmploymentType[];
