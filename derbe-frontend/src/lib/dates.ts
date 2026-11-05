const WEEKEND_DAYS = new Set([0, 6]);
const DAYS_PER_WEEK = 7;
const MILLISECONDS_PER_DAY = 86_400_000;

export function toDateInputValue(date: Date): string {
  const year = date.getFullYear();
  const month = String(date.getMonth() + 1).padStart(2, "0");
  const day = String(date.getDate()).padStart(2, "0");
  return `${year}-${month}-${day}`;
}

export function parseLocalDate(value: string): Date {
  const [year, month, day] = value.split("-").map(Number);
  return new Date(year, month - 1, day);
}

export function formatDate(value: string | null | undefined): string {
  if (!value) return "N/A";
  return new Intl.DateTimeFormat("en-GB", {
    day: "2-digit",
    month: "short",
    year: "numeric",
  }).format(parseLocalDate(value.slice(0, 10)));
}

export function workingDaysBetween(startValue: string, endValue: string): number {
  const start = parseLocalDate(startValue);
  const end = parseLocalDate(endValue);
  if (end < start) return 0;

  let count = 0;
  for (const date = new Date(start); date <= end; date.setDate(date.getDate() + 1)) {
    if (!WEEKEND_DAYS.has(date.getDay())) count += 1;
  }
  return count;
}

export function daysInMonth(year: number, month: number): number {
  return new Date(year, month + 1, 0).getDate();
}

export function dateRangeIso(year: number, month: number) {
  const first = new Date(year, month, 1);
  const last = new Date(year, month, daysInMonth(year, month));
  return { date_from: toDateInputValue(first), date_to: toDateInputValue(last) };
}

export function elapsedDays(startValue: string, endValue: string): number {
  return (
    Math.round(
      (parseLocalDate(endValue).getTime() - parseLocalDate(startValue).getTime()) /
        MILLISECONDS_PER_DAY,
    ) + 1
  );
}

export function monthOffset(year: number, month: number, delta: number) {
  const date = new Date(year, month + delta, 1);
  return { year: date.getFullYear(), month: date.getMonth() };
}

export function dayColumnWidth(dayCount: number): string {
  return `minmax(2rem, ${100 / Math.max(dayCount, DAYS_PER_WEEK)}fr)`;
}
