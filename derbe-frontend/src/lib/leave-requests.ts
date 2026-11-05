import { HALF_DAY_REDUCTION } from "@/constants/constants";
import { workingDaysBetween } from "./dates";

export function halfDayFlags(selection: HalfDaySelection) {
  return {
    first_day_half: selection === "first" || selection === "both",
    last_day_half: selection === "last" || selection === "both",
  };
}

export function leaveDuration(
  dateFrom: string,
  dateTo: string,
  selection: HalfDaySelection,
) {
  const flags = halfDayFlags(selection);
  const halfDays = Number(flags.first_day_half) + Number(flags.last_day_half);
  return Math.max(
    0,
    workingDaysBetween(dateFrom, dateTo) - halfDays * HALF_DAY_REDUCTION,
  );
}

export function halfDaySelection(values: {
  first_day_half: boolean;
  last_day_half: boolean;
}): HalfDaySelection {
  if (values.first_day_half && values.last_day_half) return "both";
  if (values.first_day_half) return "first";
  if (values.last_day_half) return "last";
  return "full";
}

export type HalfDaySelection = "full" | "first" | "last" | "both";
