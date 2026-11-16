import type { EmploymentType } from "@/bindings/EmploymentType";
import type { OthersDaysVisibility } from "@/bindings/OthersDaysVisibility";
import type { HalfDaySelection } from "@/lib/leave-requests";

export const EMPTY_VALUE = "N/A";
export const NONE_OPTION = "none";
export const HALF_DAY_REDUCTION = 0.5;
export const DEFAULT_LEAVE_COLOUR = "#f97316";
export const DEFAULT_LEAVE_TYPE_QUOTA = 0;

export const EMPLOYMENT_TYPES = [
  "Permanent",
  "Contract",
] as const satisfies readonly EmploymentType[];

export const HALF_DAY_OPTIONS: { value: HalfDaySelection; label: string }[] = [
  { value: "full", label: "Full days" },
  { value: "first", label: "First day half" },
  { value: "last", label: "Last day half" },
  { value: "both", label: "First and last day half" },
];

export const LEAVE_TYPE_FLAGS = [
  ["can_book_in_future", "Can book in future?"],
  ["default_in_leave_request", "Default in leave request?"],
  ["show_in_approve_days", "Show in approve days?"],
] as const;

export const OTHERS_DAYS_VISIBILITY_OPTIONS: {
  value: OthersDaysVisibility;
  label: string;
}[] = [
  { value: "Hidden", label: "Hidden" },
  { value: "Shown", label: "Shown" },
  { value: "ShownByDefault", label: "Shown by default" },
];
