import type { HalfDaySelection } from "@/features/leave/leave-requests";

export const HALF_DAY_REDUCTION = 0.5;

export const HALF_DAY_OPTIONS: { value: HalfDaySelection; label: string }[] = [
  { value: "full", label: "Full days" },
  { value: "first", label: "First day half" },
  { value: "last", label: "Last day half" },
  { value: "both", label: "First and last day half" },
];

export const SINGLE_DAY_HALF_DAY_OPTIONS = [
  { value: "full", label: "Full day" },
  { value: "first", label: "Half day" },
] as const;
