import { zodResolver } from "@hookform/resolvers/zod";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { format } from "date-fns";
import { CalendarDays } from "lucide-react";
import { useEffect, useState } from "react";
import type { DateRange } from "react-day-picker";
import { useForm, useWatch } from "react-hook-form";
import { useNavigate } from "react-router-dom";
import { z } from "zod";
import { createLeaveRequest, employeesQueryOptions, leaveTypesQueryOptions } from "@/api";
import type { LeaveRequestCreate } from "@/bindings/LeaveRequestCreate";
import { PageHeading } from "@/components/page-heading";
import { ErrorAlert, QueryMessage } from "@/components/query-message";
import { Button } from "@/components/ui/button";
import { Calendar } from "@/components/ui/calendar";
import { Card, CardContent } from "@/components/ui/card";
import {
  Form,
  FormControl,
  FormDescription,
  FormItem,
  FormLabel,
  FormMessage,
  FormField as RHFField,
} from "@/components/ui/form";
import { Popover, PopoverContent, PopoverTrigger } from "@/components/ui/popover";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { Textarea } from "@/components/ui/textarea";
import {
  HALF_DAY_OPTIONS,
  SINGLE_DAY_HALF_DAY_OPTIONS,
} from "@/features/leave/constants";
import { halfDayFlags, leaveDuration } from "@/features/leave/leave-requests";
import { parseLocalDate, toDateInputValue } from "@/lib/dates";
import { derbeKeys } from "@/query-keys";

const leaveRequestFormSchema = z
  .object({
    employee_unid: z
      .string()
      .nullable()
      .refine((value): boolean => value !== null, "Choose an employee."),
    leave_type_id: z
      .number()
      .int()
      .positive()
      .nullable()
      .refine((value): boolean => value !== null, "Choose a leave type."),
    date_from: z.string().min(1, "Choose a start date."),
    date_to: z.string().min(1, "Choose an end date."),
    half_days: z.enum(["full", "first", "last", "both"]),
    notes: z.string(),
  })
  .refine((values) => values.date_to >= values.date_from, {
    path: ["date_to"],
    message: "End date must be on or after start date.",
  });

type LeaveRequestFormValues = z.infer<typeof leaveRequestFormSchema>;

export function LeaveRequestFormPage() {
  const navigate = useNavigate();
  const queryClient = useQueryClient();
  const employeesQuery = useQuery(employeesQueryOptions());
  const leaveTypesQuery = useQuery(leaveTypesQueryOptions());
  const today = toDateInputValue(new Date());
  const todayDate = parseLocalDate(today);
  const form = useForm<LeaveRequestFormValues>({
    resolver: zodResolver(leaveRequestFormSchema),
    defaultValues: {
      employee_unid: null,
      leave_type_id: null,
      date_from: today,
      date_to: today,
      half_days: "full",
      notes: "",
    },
  });
  const { control, getValues, setValue } = form;
  const [datePickerOpen, setDatePickerOpen] = useState(false);
  const [selectedDateRange, setSelectedDateRange] = useState<DateRange | undefined>(
    () => ({ from: todayDate, to: todayDate }),
  );
  const dateFrom = useWatch({ control, name: "date_from" });
  const dateTo = useWatch({ control, name: "date_to" });
  const halfDays = useWatch({ control, name: "half_days" });
  const duration = leaveDuration(dateFrom, dateTo, halfDays);
  const isSingleDay = dateFrom === dateTo;

  function setFormDateRange(from: Date, to: Date) {
    setValue("date_from", toDateInputValue(from), {
      shouldDirty: true,
      shouldValidate: true,
    });
    setValue("date_to", toDateInputValue(to), {
      shouldDirty: true,
      shouldValidate: true,
    });
  }

  function handleDateRangeSelect(range: DateRange | undefined) {
    setSelectedDateRange(range);
    if (range?.from && range.to) {
      setFormDateRange(range.from, range.to);
      if (range.to > range.from) setDatePickerOpen(false);
    }
  }

  function handleDatePickerOpenChange(open: boolean) {
    setDatePickerOpen(open);
    if (open) {
      setSelectedDateRange(undefined);
      return;
    }
    if (!open && selectedDateRange?.from && !selectedDateRange.to) {
      setSelectedDateRange({ from: selectedDateRange.from, to: selectedDateRange.from });
      setFormDateRange(selectedDateRange.from, selectedDateRange.from);
    }
  }

  useEffect(() => {
    if (getValues("employee_unid") === null && employeesQuery.data?.[0]) {
      setValue("employee_unid", employeesQuery.data[0].employee_unid);
    }
    if (getValues("leave_type_id") === null && leaveTypesQuery.data?.length) {
      const defaultLeaveType =
        leaveTypesQuery.data.find((leaveType) => leaveType.default_in_leave_request) ??
        leaveTypesQuery.data[0];
      setValue("leave_type_id", defaultLeaveType.id);
    }
  }, [employeesQuery.data, getValues, leaveTypesQuery.data, setValue]);

  useEffect(() => {
    if (!isSingleDay) return;
    if (halfDays === "both") {
      setValue("half_days", "full", { shouldValidate: true });
    } else if (halfDays === "last") {
      setValue("half_days", "first", { shouldValidate: true });
    }
  }, [halfDays, isSingleDay, setValue]);

  const mutation = useMutation({
    mutationFn: (values: LeaveRequestFormValues) => {
      const requestValues = {
        date_from: values.date_from,
        date_to: values.date_to,
        ...halfDayFlags(values.half_days),
        total_duration: leaveDuration(values.date_from, values.date_to, values.half_days),
      };
      const employee_unid =
        values.employee_unid ?? employeesQuery.data?.[0]?.employee_unid;
      if (employee_unid === undefined) {
        throw new Error("Choose an employee for this leave request.");
      }
      const request: LeaveRequestCreate = {
        employee_unid,
        leave_type_id:
          values.leave_type_id ??
          leaveTypesQuery.data?.find((leaveType) => leaveType.default_in_leave_request)
            ?.id ??
          leaveTypesQuery.data?.[0]?.id ??
          0,
        leave_status: "Pending",
        values: requestValues,
        date_requested: new Date().toISOString(),
        date_finalised: null,
        notes: values.notes || null,
      };
      return createLeaveRequest(request);
    },
    onSuccess: async (request) => {
      await queryClient.invalidateQueries({ queryKey: derbeKeys.myLeaveAll() });
      navigate(`/dashboard/leave-request/${request.id}`);
    },
  });
  const loadError = employeesQuery.error ?? leaveTypesQuery.error;

  if (employeesQuery.isPending || leaveTypesQuery.isPending) return <QueryMessage />;
  if (loadError) return <QueryMessage error={loadError} />;
  const employees = employeesQuery.data ?? [];
  const leaveTypes = leaveTypesQuery.data ?? [];
  const defaultLeaveTypes = leaveTypes.filter(
    (leaveType) => leaveType.default_in_leave_request,
  );
  const requestableLeaveTypes =
    defaultLeaveTypes.length > 0 ? defaultLeaveTypes : leaveTypes;

  const availableHalfDayOptions = isSingleDay
    ? SINGLE_DAY_HALF_DAY_OPTIONS
    : HALF_DAY_OPTIONS;
  return (
    <>
      <PageHeading title="Leave Request" />
      <Card>
        <CardContent className="pt-6">
          <Form {...form}>
            <form
              className="grid max-w-2xl gap-5"
              onSubmit={form.handleSubmit((values) => mutation.mutate(values))}
            >
              <RHFField
                control={control}
                name="employee_unid"
                render={({ field }) => (
                  <FormItem>
                    <FormLabel>Employee</FormLabel>
                    <Select
                      value={field.value ? field.value : undefined}
                      onValueChange={(value) => field.onChange(value)}
                    >
                      <FormControl>
                        <SelectTrigger>
                          <SelectValue placeholder="Select employee" />
                        </SelectTrigger>
                      </FormControl>
                      <SelectContent>
                        {employees.map((employee) => (
                          <SelectItem
                            key={employee.employee_unid}
                            value={employee.employee_unid}
                          >
                            {employee.first_name} {employee.last_name}
                          </SelectItem>
                        ))}
                      </SelectContent>
                    </Select>
                    <FormMessage />
                  </FormItem>
                )}
              />
              <RHFField
                control={control}
                name="leave_type_id"
                render={({ field }) => (
                  <FormItem>
                    <FormLabel>Leave Type</FormLabel>
                    <Select
                      value={field.value ? String(field.value) : undefined}
                      onValueChange={(value) => field.onChange(Number(value))}
                    >
                      <FormControl>
                        <SelectTrigger>
                          <SelectValue placeholder="Select leave type" />
                        </SelectTrigger>
                      </FormControl>
                      <SelectContent>
                        {requestableLeaveTypes.map((leaveType) => (
                          <SelectItem key={leaveType.id} value={String(leaveType.id)}>
                            {leaveType.name}
                          </SelectItem>
                        ))}
                      </SelectContent>
                    </Select>
                    <FormMessage />
                  </FormItem>
                )}
              />
              <RHFField
                control={control}
                name="date_from"
                render={({ field }) => (
                  <FormItem>
                    <FormLabel>Date Range</FormLabel>
                    <Popover
                      open={datePickerOpen}
                      onOpenChange={handleDatePickerOpenChange}
                    >
                      <FormControl>
                        <PopoverTrigger asChild>
                          <Button
                            ref={field.ref}
                            type="button"
                            variant="outline"
                            aria-invalid={Boolean(
                              form.formState.errors.date_from ||
                                form.formState.errors.date_to,
                            )}
                            className="w-full justify-start text-left font-normal"
                            onBlur={field.onBlur}
                          >
                            <CalendarDays />
                            {dateFrom === dateTo
                              ? format(parseLocalDate(dateFrom), "dd/MM/yyyy")
                              : `${format(parseLocalDate(dateFrom), "dd/MM/yyyy")} – ${format(parseLocalDate(dateTo), "dd/MM/yyyy")}`}
                          </Button>
                        </PopoverTrigger>
                      </FormControl>
                      <PopoverContent align="start" className="w-auto p-0">
                        <Calendar
                          mode="range"
                          selected={selectedDateRange}
                          onSelect={handleDateRangeSelect}
                          disabled={{ before: todayDate }}
                          defaultMonth={parseLocalDate(dateFrom)}
                          autoFocus
                        />
                      </PopoverContent>
                    </Popover>
                    <FormMessage />
                    {form.formState.errors.date_to?.message && (
                      <p className="text-sm font-medium text-destructive" role="alert">
                        {form.formState.errors.date_to.message}
                      </p>
                    )}
                  </FormItem>
                )}
              />
              <RHFField
                control={control}
                name="half_days"
                render={({ field }) => (
                  <FormItem>
                    <FormLabel>Half Day Options</FormLabel>
                    <FormDescription>
                      {isSingleDay
                        ? "Choose a full day or half day."
                        : "Choose a half day on either end of the request."}
                    </FormDescription>
                    <Select value={field.value} onValueChange={field.onChange}>
                      <FormControl>
                        <SelectTrigger>
                          <SelectValue />
                        </SelectTrigger>
                      </FormControl>
                      <SelectContent>
                        {availableHalfDayOptions.map((option) => (
                          <SelectItem key={option.value} value={option.value}>
                            {option.label}
                          </SelectItem>
                        ))}
                      </SelectContent>
                    </Select>
                    <FormMessage />
                  </FormItem>
                )}
              />
              <div className="rounded-md bg-muted px-3 py-2 text-sm">
                Requested duration:{" "}
                <strong>
                  {duration} {duration === 1 ? "day" : "days"}
                </strong>
              </div>
              <RHFField
                control={control}
                name="notes"
                render={({ field }) => (
                  <FormItem>
                    <FormLabel>Notes</FormLabel>
                    <FormControl>
                      <Textarea placeholder="Notes..." {...field} />
                    </FormControl>
                    <FormDescription>Optional</FormDescription>
                    <FormMessage />
                  </FormItem>
                )}
              />
              {mutation.error && <ErrorAlert error={mutation.error} />}
              <div className="flex gap-2">
                <Button
                  type="button"
                  variant="secondary"
                  onClick={() => navigate("/dashboard/my-leave")}
                >
                  Cancel
                </Button>
                <Button
                  type="submit"
                  disabled={
                    mutation.isPending ||
                    duration <= 0 ||
                    employees.length === 0 ||
                    leaveTypes.length === 0
                  }
                >
                  {mutation.isPending ? "Submitting…" : "Submit"}
                </Button>
              </div>
            </form>
          </Form>
        </CardContent>
      </Card>
    </>
  );
}
