import { useQuery } from "@tanstack/react-query";
import { ArrowLeft, ArrowRight } from "lucide-react";
import { useState } from "react";
import { allLeaveQueryOptions } from "@/api";
import { PageHeading } from "@/components/PageHeading";
import { QueryMessage } from "@/components/QueryMessage";
import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/card";
import { daysInMonth, monthOffset, parseLocalDate } from "@/lib/dates";

export function AllLeavePage() {
  const [currentMonth, setCurrentMonth] = useState(() => ({
    year: new Date().getFullYear(),
    month: new Date().getMonth(),
  }));
  const query = useQuery(allLeaveQueryOptions(currentMonth.year, currentMonth.month));
  const dayCount = daysInMonth(currentMonth.year, currentMonth.month);
  const monthLabel = new Intl.DateTimeFormat("en-GB", {
    month: "long",
    year: "numeric",
  }).format(new Date(currentMonth.year, currentMonth.month, 1));

  function changeMonth(offset: number) {
    setCurrentMonth((month) => monthOffset(month.year, month.month, offset));
  }

  return (
    <>
      <PageHeading
        title="All Leave"
        description="Team leave calendar."
        action={
          <div className="flex items-center gap-2">
            <Button
              size="icon"
              variant="outline"
              aria-label="Previous month"
              onClick={() => changeMonth(-1)}
            >
              <ArrowLeft />
            </Button>
            <span className="min-w-36 text-center font-medium">{monthLabel}</span>
            <Button
              size="icon"
              variant="outline"
              aria-label="Next month"
              onClick={() => changeMonth(1)}
            >
              <ArrowRight />
            </Button>
          </div>
        }
      />
      {query.isPending || query.isError ? (
        <QueryMessage error={query.error} />
      ) : (
        <Card className="overflow-hidden">
          <div className="overflow-x-auto">
            <div className="min-w-max">
              <div
                className="grid border-b bg-muted/60"
                style={{
                  gridTemplateColumns: `12rem repeat(${dayCount}, minmax(2rem, 1fr))`,
                }}
              >
                <div className="sticky left-0 z-10 bg-muted/60 px-3 py-2 text-sm font-medium">
                  Employee
                </div>
                {Array.from({ length: dayCount }, (_, index) => index + 1).map((day) => (
                  <div
                    key={day}
                    className="px-1 py-2 text-center text-xs text-muted-foreground"
                  >
                    {day}
                  </div>
                ))}
              </div>
              {query.data.map((employee) => (
                <div
                  key={employee.employee_name}
                  className="grid border-b last:border-b-0"
                  style={{
                    gridTemplateColumns: `12rem repeat(${dayCount}, minmax(2rem, 1fr))`,
                  }}
                >
                  <div className="sticky left-0 z-10 row-span-2 flex items-center border-r bg-card px-3 py-2 text-sm font-medium">
                    {employee.employee_name}
                  </div>
                  {(["upper", "lower"] as const).map((row) =>
                    Array.from({ length: dayCount }, (_, index) => index + 1).map(
                      (day) => {
                        const date = new Date(currentMonth.year, currentMonth.month, day);
                        const active = employee.leaves.find((leave) => {
                          const from = parseLocalDate(leave.date_from);
                          const to = parseLocalDate(leave.date_to);
                          return date >= from && date <= to;
                        });
                        const half =
                          active &&
                          ((active.first_day_half &&
                            day === Number(active.date_from.slice(8, 10))) ||
                            (active.last_day_half &&
                              day === Number(active.date_to.slice(8, 10))));
                        const visible = active && (!half || (half && row === "lower"));
                        return (
                          <div
                            key={`${row}-${day}`}
                            title={
                              active
                                ? `${active.date_from} to ${active.date_to}`
                                : undefined
                            }
                            className="h-5 border-r border-b last:border-r-0"
                            style={{
                              backgroundColor: visible ? active.colour : undefined,
                            }}
                          />
                        );
                      },
                    ),
                  )}
                </div>
              ))}
              {query.data.length === 0 && (
                <p className="p-4 text-sm text-muted-foreground">
                  No leave to show this month.
                </p>
              )}
            </div>
          </div>
        </Card>
      )}
    </>
  );
}
