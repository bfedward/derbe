import type { OthersDaysVisibility } from "@/bindings/OthersDaysVisibility";

export const EMPTY_VALUE = "N/A";
export const DEFAULT_LEAVE_COLOUR = "#f97316";
export const DEFAULT_LEAVE_TYPE_QUOTA = 0;

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
