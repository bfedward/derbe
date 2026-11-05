import { zodResolver } from "@hookform/resolvers/zod";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { useEffect } from "react";
import { useForm, useWatch } from "react-hook-form";
import { useNavigate } from "react-router-dom";
import { z } from "zod";
import {
  createLeaveRequest,
  DEFAULT_EMPLOYEE_ID,
  employeesQueryOptions,
  leaveTypesQueryOptions,
} from "@/api";
import type { LeaveRequestCreate } from "@/bindings/LeaveRequestCreate";
import { PageHeading } from "@/components/PageHeading";
import { ErrorAlert, QueryMessage } from "@/components/QueryMessage";
import { Button } from "@/components/ui/button";
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
import { Input } from "@/components/ui/input";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { Textarea } from "@/components/ui/textarea";
import { HALF_DAY_OPTIONS } from "@/constants/constants";
import { elapsedDays, toDateInputValue } from "@/lib/dates";
import { halfDayFlags, leaveDuration } from "@/lib/leave-requests";
import { derbeKeys } from "@/query-keys";

const leaveRequestFormSchema = z
  .object({
    employee_id: z
      .number()
      .int()
      .positive()
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
  const form = useForm<LeaveRequestFormValues>({
    resolver: zodResolver(leaveRequestFormSchema),
    defaultValues: {
      employee_id: null,
      leave_type_id: null,
      date_from: today,
      date_to: today,
      half_days: "full",
      notes: "",
    },
  });
  const { control, getValues, setValue } = form;
  const dateFrom = useWatch({ control, name: "date_from" });
  const dateTo = useWatch({ control, name: "date_to" });
  const halfDays = useWatch({ control, name: "half_days" });
  const duration = leaveDuration(dateFrom, dateTo, halfDays);

  useEffect(() => {
    if (getValues("employee_id") === null && employeesQuery.data?.[0]) {
      setValue("employee_id", employeesQuery.data[0].id);
    }
    if (getValues("leave_type_id") === null && leaveTypesQuery.data?.length) {
      const defaultLeaveType =
        leaveTypesQuery.data.find((leaveType) => leaveType.default_in_leave_request) ??
        leaveTypesQuery.data[0];
      setValue("leave_type_id", defaultLeaveType.id);
    }
  }, [employeesQuery.data, getValues, leaveTypesQuery.data, setValue]);

  const mutation = useMutation({
    mutationFn: (values: LeaveRequestFormValues) => {
      const requestValues = {
        date_from: values.date_from,
        date_to: values.date_to,
        ...halfDayFlags(values.half_days),
        total_duration: leaveDuration(values.date_from, values.date_to, values.half_days),
      };
      const request: LeaveRequestCreate = {
        employee_id:
          values.employee_id ?? employeesQuery.data?.[0]?.id ?? DEFAULT_EMPLOYEE_ID,
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
      navigate(`/leave-request/${request.id}`);
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

  const availableHalfDayOptions =
    elapsedDays(dateFrom, dateTo) <= 1 ? HALF_DAY_OPTIONS.slice(0, 3) : HALF_DAY_OPTIONS;
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
                name="employee_id"
                render={({ field }) => (
                  <FormItem>
                    <FormLabel>Employee</FormLabel>
                    <Select
                      value={field.value ? String(field.value) : undefined}
                      onValueChange={(value) => field.onChange(Number(value))}
                    >
                      <FormControl>
                        <SelectTrigger>
                          <SelectValue placeholder="Select employee" />
                        </SelectTrigger>
                      </FormControl>
                      <SelectContent>
                        {employees.map((employee) => (
                          <SelectItem key={employee.id} value={String(employee.id)}>
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
              <div className="grid gap-4 sm:grid-cols-2">
                <RHFField
                  control={control}
                  name="date_from"
                  render={({ field }) => (
                    <FormItem>
                      <FormLabel>Date From</FormLabel>
                      <FormControl>
                        <Input type="date" min={today} {...field} />
                      </FormControl>
                      <FormMessage />
                    </FormItem>
                  )}
                />
                <RHFField
                  control={control}
                  name="date_to"
                  render={({ field }) => (
                    <FormItem>
                      <FormLabel>Date To</FormLabel>
                      <FormControl>
                        <Input type="date" min={dateFrom} {...field} />
                      </FormControl>
                      <FormMessage />
                    </FormItem>
                  )}
                />
              </div>
              <RHFField
                control={control}
                name="half_days"
                render={({ field }) => (
                  <FormItem>
                    <FormLabel>Half Day Options</FormLabel>
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
                  onClick={() => navigate("/my-leave")}
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
