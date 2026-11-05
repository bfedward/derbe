import { zodResolver } from "@hookform/resolvers/zod";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { useEffect } from "react";
import { useForm, useWatch } from "react-hook-form";
import { useNavigate, useParams } from "react-router-dom";
import { z } from "zod";
import {
  leaveRequestQueryOptions,
  updateLeaveRequest,
  updateLeaveRequestStatus,
} from "@/api";
import type { LeaveRequestStatusUpdate } from "@/bindings/LeaveRequestStatusUpdate";
import type { LeaveRequestUpdate } from "@/bindings/LeaveRequestUpdate";
import type { LeaveStatus } from "@/bindings/LeaveStatus";
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
import { formatDate } from "@/lib/dates";
import { halfDayFlags, halfDaySelection, leaveDuration } from "@/lib/leave-requests";
import { derbeKeys } from "@/query-keys";

const leaveRequestEditSchema = z
  .object({
    date_from: z.string().min(1, "Choose a start date."),
    date_to: z.string().min(1, "Choose an end date."),
    half_days: z.enum(["full", "first", "last", "both"]),
    notes: z.string(),
  })
  .refine((values) => values.date_to >= values.date_from, {
    path: ["date_to"],
    message: "End date must be on or after start date.",
  });

type LeaveRequestEditValues = z.infer<typeof leaveRequestEditSchema>;

export function LeaveRequestProfilePage() {
  const { id } = useParams();
  const requestId = Number(id);
  const queryClient = useQueryClient();
  const navigate = useNavigate();
  const query = useQuery({
    ...leaveRequestQueryOptions(requestId),
    enabled: Number.isInteger(requestId) && requestId > 0,
  });
  const form = useForm<LeaveRequestEditValues>({
    resolver: zodResolver(leaveRequestEditSchema),
    defaultValues: { date_from: "", date_to: "", half_days: "full", notes: "" },
  });
  const { control, reset } = form;
  const dateFrom = useWatch({ control, name: "date_from" });
  const dateTo = useWatch({ control, name: "date_to" });
  const halfDays = useWatch({ control, name: "half_days" });

  useEffect(() => {
    if (!query.data) return;
    reset({
      date_from: query.data.date_from,
      date_to: query.data.date_to,
      half_days: halfDaySelection(query.data),
      notes: query.data.notes ?? "",
    });
  }, [query.data, reset]);

  const updateMutation = useMutation({
    mutationFn: (values: LeaveRequestEditValues) => {
      if (!query.data) throw new Error("Leave request not loaded.");
      const input: LeaveRequestUpdate = {
        employee_id: query.data.employee_id,
        leave_type_id: query.data.leave_type_id,
        leave_status: query.data.leave_status,
        values: {
          date_from: values.date_from,
          date_to: values.date_to,
          ...halfDayFlags(values.half_days),
          total_duration: leaveDuration(
            values.date_from,
            values.date_to,
            values.half_days,
          ),
        },
        date_requested: query.data.date_requested,
        date_finalised: query.data.date_finalised,
        notes: values.notes || null,
      };
      return updateLeaveRequest(requestId, input);
    },
    onSuccess: async () => {
      await queryClient.invalidateQueries({
        queryKey: derbeKeys.leaveRequest(requestId),
      });
      await queryClient.invalidateQueries({ queryKey: derbeKeys.myLeaveAll() });
      navigate("/my-leave");
    },
  });
  const statusMutation = useMutation({
    mutationFn: (status: LeaveStatus) => {
      const input: LeaveRequestStatusUpdate = {
        leave_status: status,
        notes: form.getValues("notes") || null,
      };
      return updateLeaveRequestStatus(requestId, input);
    },
    onSuccess: async () => {
      await queryClient.invalidateQueries({
        queryKey: derbeKeys.leaveRequest(requestId),
      });
      await queryClient.invalidateQueries({ queryKey: derbeKeys.myLeaveAll() });
      navigate("/my-leave");
    },
  });

  if (!Number.isInteger(requestId) || requestId <= 0)
    return <QueryMessage empty="Leave request not found." />;
  if (query.isPending || query.isError) return <QueryMessage error={query.error} />;
  const request = query.data;
  const duration = leaveDuration(dateFrom, dateTo, halfDays);
  const saveError = updateMutation.error ?? statusMutation.error;
  return (
    <>
      <PageHeading
        title="Leave Request Details"
        description={`Request ${request.id} · ${request.leave_status}`}
      />
      <Card>
        <CardContent className="pt-6">
          <Form {...form}>
            <form
              className="grid max-w-2xl gap-5"
              onSubmit={form.handleSubmit((values) => updateMutation.mutate(values))}
            >
              <div className="grid gap-2">
                <span className="text-sm font-medium">Employee</span>
                <Input readOnly value={`Employee #${request.employee_id}`} />
              </div>
              <div className="grid gap-2">
                <span className="text-sm font-medium">Leave Type</span>
                <Input readOnly value={`Leave type #${request.leave_type_id}`} />
              </div>
              <div className="grid gap-4 sm:grid-cols-2">
                <RHFField
                  control={control}
                  name="date_from"
                  render={({ field }) => (
                    <FormItem>
                      <FormLabel>Date From</FormLabel>
                      <FormControl>
                        <Input type="date" {...field} />
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
                        {HALF_DAY_OPTIONS.map((option) => (
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
              <p className="text-sm text-muted-foreground">
                Requested {formatDate(request.date_requested.slice(0, 10))}
                {request.date_finalised
                  ? ` · Finalised ${formatDate(request.date_finalised.slice(0, 10))}`
                  : ""}
              </p>
              {saveError && <ErrorAlert error={saveError} />}
              <div className="flex flex-wrap gap-2">
                <Button
                  type="button"
                  variant="secondary"
                  onClick={() => navigate("/my-leave")}
                >
                  Cancel
                </Button>
                <Button
                  type="submit"
                  disabled={updateMutation.isPending || duration <= 0}
                >
                  {updateMutation.isPending ? "Saving…" : "Submit"}
                </Button>
              </div>
              <div className="flex flex-wrap gap-2 border-t pt-4">
                <Button
                  type="button"
                  variant="outline"
                  onClick={() => statusMutation.mutate("MoreInfoReq")}
                  disabled={statusMutation.isPending}
                >
                  More Info Required
                </Button>
                <Button
                  type="button"
                  variant="destructive"
                  onClick={() => statusMutation.mutate("Rejected")}
                  disabled={statusMutation.isPending}
                >
                  Reject
                </Button>
                <Button
                  type="button"
                  onClick={() => statusMutation.mutate("Accepted")}
                  disabled={statusMutation.isPending}
                >
                  Accept
                </Button>
              </div>
            </form>
          </Form>
        </CardContent>
      </Card>
    </>
  );
}
