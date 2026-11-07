import { zodResolver } from "@hookform/resolvers/zod";
import { useMutation, useQueryClient } from "@tanstack/react-query";
import { useForm } from "react-hook-form";
import { useNavigate } from "react-router-dom";
import { z } from "zod";
import { createLeaveType } from "@/api";
import { PageHeading } from "@/components/page-heading";
import { ErrorAlert } from "@/components/query-message";
import { Button } from "@/components/ui/button";
import { Card, CardContent } from "@/components/ui/card";
import { Checkbox } from "@/components/ui/checkbox";
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
  DEFAULT_LEAVE_COLOUR,
  DEFAULT_LEAVE_TYPE_QUOTA,
  LEAVE_TYPE_FLAGS,
} from "@/constants/constants";
import { derbeKeys } from "@/query-keys";

const leaveTypeFormSchema = z.object({
  name: z.string().trim().min(1, "Leave type name is required."),
  default_quota: z.number().min(DEFAULT_LEAVE_TYPE_QUOTA).nullable(),
  colour: z.string().regex(/^#[\da-f]{6}$/i, "Choose a valid colour."),
  can_book_in_future: z.boolean(),
  show_in_others_days: z.boolean(),
  default_in_others_days: z.boolean(),
  default_in_leave_request: z.boolean(),
  show_in_approve_days: z.boolean(),
});

type LeaveTypeFormValues = z.infer<typeof leaveTypeFormSchema>;

export function NewLeaveTypePage() {
  const navigate = useNavigate();
  const queryClient = useQueryClient();
  const form = useForm<LeaveTypeFormValues>({
    resolver: zodResolver(leaveTypeFormSchema),
    defaultValues: {
      name: "",
      default_quota: null,
      colour: DEFAULT_LEAVE_COLOUR,
      can_book_in_future: false,
      show_in_others_days: false,
      default_in_others_days: false,
      default_in_leave_request: false,
      show_in_approve_days: false,
    },
  });
  const mutation = useMutation({
    mutationFn: (values: LeaveTypeFormValues) => createLeaveType(values),
    onSuccess: async () => {
      await queryClient.invalidateQueries({ queryKey: derbeKeys.leaveTypes() });
      navigate("/leave-types");
    },
  });

  return (
    <>
      <PageHeading title="New Leave Type" />
      <Card>
        <CardContent className="pt-6">
          <Form {...form}>
            <form
              className="grid max-w-2xl gap-5"
              onSubmit={form.handleSubmit((values) => mutation.mutate(values))}
            >
              <RHFField
                control={form.control}
                name="name"
                render={({ field }) => (
                  <FormItem>
                    <FormLabel>Name</FormLabel>
                    <FormControl>
                      <Input {...field} />
                    </FormControl>
                    <FormMessage />
                  </FormItem>
                )}
              />
              <RHFField
                control={form.control}
                name="default_quota"
                render={({ field }) => (
                  <FormItem>
                    <FormLabel>Default Quota</FormLabel>
                    <FormControl>
                      <Input
                        type="number"
                        min={DEFAULT_LEAVE_TYPE_QUOTA}
                        step="any"
                        value={field.value ?? ""}
                        onChange={(event) =>
                          field.onChange(
                            event.target.value === "" ? null : Number(event.target.value),
                          )
                        }
                      />
                    </FormControl>
                    <FormDescription>Optional number of days.</FormDescription>
                    <FormMessage />
                  </FormItem>
                )}
              />
              <RHFField
                control={form.control}
                name="colour"
                render={({ field }) => (
                  <FormItem>
                    <FormLabel>Colour</FormLabel>
                    <div className="flex items-center gap-3">
                      <FormControl>
                        <Input type="color" className="w-16 p-1" {...field} />
                      </FormControl>
                      <span className="text-sm text-muted-foreground">{field.value}</span>
                    </div>
                    <FormMessage />
                  </FormItem>
                )}
              />
              <div className="grid gap-3 rounded-lg border p-4">
                {LEAVE_TYPE_FLAGS.map(([key, label]) => (
                  <RHFField
                    key={key}
                    control={form.control}
                    name={key}
                    render={({ field }) => (
                      <FormItem className="flex items-center gap-3 space-y-0">
                        <FormControl>
                          <Checkbox
                            aria-label={label}
                            checked={field.value}
                            onCheckedChange={field.onChange}
                          />
                        </FormControl>
                        <span className="text-sm">{label}</span>
                      </FormItem>
                    )}
                  />
                ))}
              </div>
              {mutation.error && <ErrorAlert error={mutation.error} />}
              <div className="flex gap-2">
                <Button
                  type="button"
                  variant="secondary"
                  onClick={() => navigate("/leave-types")}
                >
                  Cancel
                </Button>
                <Button type="submit" disabled={mutation.isPending}>
                  {mutation.isPending ? "Saving…" : "Submit"}
                </Button>
              </div>
            </form>
          </Form>
        </CardContent>
      </Card>
    </>
  );
}
