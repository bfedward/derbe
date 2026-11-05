import { zodResolver } from "@hookform/resolvers/zod";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { ArrowLeft, ArrowRight, Pencil, Plus } from "lucide-react";
import { useEffect, useState } from "react";
import { useForm, useWatch } from "react-hook-form";
import { Link, useNavigate, useParams } from "react-router-dom";
import { z } from "zod";
import {
  allLeaveQueryOptions,
  createEmployee,
  createLeaveRequest,
  createLeaveType,
  DEFAULT_EMPLOYEE_ID,
  divisionsQueryOptions,
  employeeQueryOptions,
  employeesQueryOptions,
  leaveRequestQueryOptions,
  leaveTypesQueryOptions,
  locationsQueryOptions,
  myLeaveQueryOptions,
  updateEmployee,
  updateLeaveRequest,
  updateLeaveRequestStatus,
} from "@/api";
import type { EmploymentType } from "@/bindings/EmploymentType";
import type { LeaveRequestCreate } from "@/bindings/LeaveRequestCreate";
import type { LeaveRequestStatusUpdate } from "@/bindings/LeaveRequestStatusUpdate";
import type { LeaveRequestUpdate } from "@/bindings/LeaveRequestUpdate";
import type { LeaveStatus } from "@/bindings/LeaveStatus";
import { Alert, AlertDescription, AlertTitle } from "@/components/ui/alert";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
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
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { Skeleton } from "@/components/ui/skeleton";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import { Textarea } from "@/components/ui/textarea";
import {
  daysInMonth,
  elapsedDays,
  formatDate,
  monthOffset,
  parseLocalDate,
  toDateInputValue,
  workingDaysBetween,
} from "@/lib/dates";
import { derbeKeys } from "@/query-keys";

const EMPTY_VALUE = "N/A";
const NONE_OPTION = "none";
const HALF_DAY_REDUCTION = 0.5;
const DEFAULT_LEAVE_COLOUR = "#f97316";
const DEFAULT_LEAVE_TYPE_QUOTA = 0;
const EMPLOYMENT_TYPES = [
  "Permanent",
  "Contract",
] as const satisfies readonly EmploymentType[];

function PageHeading({
  title,
  description,
  action,
}: {
  title: string;
  description?: string;
  action?: React.ReactNode;
}) {
  return (
    <div className="flex flex-wrap items-start justify-between gap-3">
      <div>
        <h1 className="text-2xl font-semibold tracking-tight">{title}</h1>
        {description && (
          <p className="mt-1 text-sm text-muted-foreground">{description}</p>
        )}
      </div>
      {action}
    </div>
  );
}

function ErrorAlert({ error }: { error: Error }) {
  return (
    <Alert variant="destructive">
      <AlertTitle>Something went wrong</AlertTitle>
      <AlertDescription>{error.message}</AlertDescription>
    </Alert>
  );
}

function QueryMessage({ error, empty }: { error?: Error | null; empty?: string }) {
  if (error) return <ErrorAlert error={error} />;
  if (empty) return <p className="text-sm text-muted-foreground">{empty}</p>;
  return (
    <output aria-label="Loading" className="grid gap-3">
      <Skeleton className="h-8 w-48" />
      <Skeleton className="h-36 w-full" />
    </output>
  );
}

export function HomePage() {
  return (
    <>
      <PageHeading
        title="Derbe"
        description="Manage leave, employees, and leave types."
      />
      <div className="grid gap-4 sm:grid-cols-3">
        {[
          ["My Leave", "/my-leave", "View requests and request days off."],
          ["All Leave", "/all-leave", "See the team leave calendar."],
          ["Organisation", "/organisation", "Manage employee records."],
        ].map(([title, href, description]) => (
          <Card key={href} className="transition-colors hover:border-primary/40">
            <CardHeader>
              <CardTitle>{title}</CardTitle>
              <CardDescription>{description}</CardDescription>
            </CardHeader>
            <CardContent>
              <Button variant="outline" asChild>
                <Link to={href}>Open</Link>
              </Button>
            </CardContent>
          </Card>
        ))}
      </div>
    </>
  );
}

export function MyLeavePage() {
  const year = new Date().getFullYear();
  const query = useQuery(myLeaveQueryOptions(year));
  return (
    <>
      <PageHeading
        title="My Leave"
        action={
          <Button asChild>
            <Link to="/leave-request">
              <Plus />
              Request Days Off
            </Link>
          </Button>
        }
      />
      {query.isPending || query.isError ? (
        <QueryMessage error={query.error} />
      ) : (
        <Card>
          <Table>
            <TableHeader>
              <TableRow>
                <TableHead>Leave Type</TableHead>
                <TableHead>Status</TableHead>
                <TableHead>Date From</TableHead>
                <TableHead>Date To</TableHead>
                <TableHead>Total Duration</TableHead>
                <TableHead />
              </TableRow>
            </TableHeader>
            <TableBody>
              {query.data.map((leave) => (
                <TableRow key={leave.leave_request_id}>
                  <TableCell className="font-medium">{leave.leave_type}</TableCell>
                  <TableCell>
                    <StatusBadge status={leave.leave_status} />
                  </TableCell>
                  <TableCell>{formatDate(leave.date_from)}</TableCell>
                  <TableCell>{formatDate(leave.date_to)}</TableCell>
                  <TableCell>{leave.total_duration}</TableCell>
                  <TableCell>
                    <Button
                      size="icon"
                      variant="ghost"
                      aria-label="Edit leave request"
                      asChild
                    >
                      <Link to={`/leave-request/${leave.leave_request_id}`}>
                        <Pencil />
                      </Link>
                    </Button>
                  </TableCell>
                </TableRow>
              ))}
            </TableBody>
          </Table>
          {query.data.length === 0 && (
            <CardContent>
              <p className="text-sm text-muted-foreground">
                No leave requests for this year.
              </p>
            </CardContent>
          )}
        </Card>
      )}
    </>
  );
}

function StatusBadge({ status }: { status: LeaveStatus }) {
  const variant =
    status === "Accepted"
      ? "accepted"
      : status === "Rejected" || status === "Cancelled"
        ? "rejected"
        : status === "Pending" || status === "MoreInfoReq"
          ? "pending"
          : "outline";
  const label = status === "MoreInfoReq" ? "More Info Required" : status;
  return <Badge variant={variant}>{label}</Badge>;
}

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

export function OrganisationPage() {
  const employeesQuery = useQuery(employeesQueryOptions());
  const divisionsQuery = useQuery(divisionsQueryOptions());
  const locationsQuery = useQuery(locationsQueryOptions());
  const error = employeesQuery.error ?? divisionsQuery.error ?? locationsQuery.error;
  const pending =
    employeesQuery.isPending || divisionsQuery.isPending || locationsQuery.isPending;
  const employees = employeesQuery.data ?? [];
  return (
    <>
      <PageHeading
        title="Organisation"
        action={
          <Button asChild>
            <Link to="/employee/new">
              <Plus />
              Create Employee
            </Link>
          </Button>
        }
      />
      {pending || error ? (
        <QueryMessage error={error} />
      ) : (
        <Card>
          <Table>
            <TableHeader>
              <TableRow>
                <TableHead>Name</TableHead>
                <TableHead>Job Title</TableHead>
                <TableHead>Employment Type</TableHead>
                <TableHead>Division</TableHead>
                <TableHead>Location</TableHead>
                <TableHead>Manager</TableHead>
                <TableHead>Email</TableHead>
                <TableHead>Phone</TableHead>
              </TableRow>
            </TableHeader>
            <TableBody>
              {employees.map((employee) => {
                const manager = employees.find(
                  (candidate) => candidate.id === employee.manager_id,
                );
                const division = divisionsQuery.data?.find(
                  (candidate) => candidate.id === employee.division_id,
                );
                const location = locationsQuery.data?.find(
                  (candidate) => candidate.id === employee.location_id,
                );
                return (
                  <TableRow key={employee.id}>
                    <TableCell className="font-medium">
                      <Link
                        className="text-primary hover:underline"
                        to={`/employee/${employee.id}`}
                      >
                        {employee.first_name} {employee.last_name}
                      </Link>
                    </TableCell>
                    <TableCell>{employee.job_title}</TableCell>
                    <TableCell>{employee.employment_type}</TableCell>
                    <TableCell>{division?.name ?? EMPTY_VALUE}</TableCell>
                    <TableCell>{location?.name ?? EMPTY_VALUE}</TableCell>
                    <TableCell>
                      {manager
                        ? `${manager.first_name} ${manager.last_name}`
                        : EMPTY_VALUE}
                    </TableCell>
                    <TableCell>{employee.email}</TableCell>
                    <TableCell>{employee.phone ?? EMPTY_VALUE}</TableCell>
                  </TableRow>
                );
              })}
            </TableBody>
          </Table>
          {employees.length === 0 && (
            <CardContent>
              <p className="text-sm text-muted-foreground">No employees yet.</p>
            </CardContent>
          )}
        </Card>
      )}
    </>
  );
}

const employeeFormSchema = z.object({
  first_name: z.string().trim().min(1, "First name is required."),
  last_name: z.string().trim().min(1, "Last name is required."),
  job_title: z.string().trim().min(1, "Job title is required."),
  division_id: z.number().int().positive("Choose a division."),
  employment_type: z.enum(EMPLOYMENT_TYPES),
  location_id: z.number().int().positive("Choose a location."),
  manager_id: z.number().int().positive().nullable(),
  email: z.email("Enter a valid email address."),
  phone: z.string(),
});

type EmployeeFormValues = z.infer<typeof employeeFormSchema>;

const EMPTY_EMPLOYEE: EmployeeFormValues = {
  first_name: "",
  last_name: "",
  job_title: "",
  division_id: 0,
  employment_type: "Permanent",
  location_id: 0,
  manager_id: null,
  email: "",
  phone: "",
};

export function EmployeeFormPage() {
  return <EmployeeForm />;
}

export function EmployeeProfilePage() {
  const { id } = useParams();
  const employeeId = Number(id);
  return Number.isInteger(employeeId) && employeeId > 0 ? (
    <EmployeeForm id={employeeId} />
  ) : (
    <QueryMessage empty="Employee not found." />
  );
}

function EmployeeForm({ id }: { id?: number }) {
  const navigate = useNavigate();
  const queryClient = useQueryClient();
  const form = useForm<EmployeeFormValues>({
    resolver: zodResolver(employeeFormSchema),
    defaultValues: EMPTY_EMPLOYEE,
  });
  const { reset } = form;
  const employeeQuery = useQuery({
    ...employeeQueryOptions(id ?? DEFAULT_EMPLOYEE_ID),
    enabled: id !== undefined,
  });
  const employeesQuery = useQuery(employeesQueryOptions());
  const divisionsQuery = useQuery(divisionsQueryOptions());
  const locationsQuery = useQuery(locationsQueryOptions());

  useEffect(() => {
    if (!employeeQuery.data) return;
    const employee = employeeQuery.data;
    reset({
      first_name: employee.first_name,
      last_name: employee.last_name,
      job_title: employee.job_title,
      division_id: employee.division_id,
      employment_type: employee.employment_type,
      location_id: employee.location_id,
      manager_id: employee.manager_id,
      email: employee.email,
      phone: employee.phone ?? "",
    });
  }, [employeeQuery.data, reset]);

  const saveMutation = useMutation({
    mutationFn: (values: EmployeeFormValues) => {
      const payload = { ...values, phone: values.phone || null };
      return id === undefined ? createEmployee(payload) : updateEmployee(id, payload);
    },
    onSuccess: async () => {
      await queryClient.invalidateQueries({ queryKey: derbeKeys.employees() });
      await queryClient.invalidateQueries({ queryKey: derbeKeys.employeeDetails() });
      navigate("/organisation");
    },
  });

  if (
    (id !== undefined && employeeQuery.isPending) ||
    employeesQuery.isPending ||
    divisionsQuery.isPending ||
    locationsQuery.isPending
  )
    return <QueryMessage />;
  const error =
    employeeQuery.error ??
    employeesQuery.error ??
    divisionsQuery.error ??
    locationsQuery.error;
  if (error) return <QueryMessage error={error} />;
  const employees = employeesQuery.data ?? [];
  const divisions = divisionsQuery.data ?? [];
  const locations = locationsQuery.data ?? [];

  return (
    <>
      <PageHeading
        title={id === undefined ? "New Employee Details" : "Employee Details"}
      />
      <Card>
        <CardContent className="pt-6">
          <Form {...form}>
            <form
              className="grid max-w-2xl gap-5"
              onSubmit={form.handleSubmit((values) => saveMutation.mutate(values))}
            >
              <RHFField
                control={form.control}
                name="first_name"
                render={({ field }) => (
                  <FormItem>
                    <FormLabel>First Name</FormLabel>
                    <FormControl>
                      <Input {...field} />
                    </FormControl>
                    <FormMessage />
                  </FormItem>
                )}
              />
              <RHFField
                control={form.control}
                name="last_name"
                render={({ field }) => (
                  <FormItem>
                    <FormLabel>Last Name</FormLabel>
                    <FormControl>
                      <Input {...field} />
                    </FormControl>
                    <FormMessage />
                  </FormItem>
                )}
              />
              <RHFField
                control={form.control}
                name="job_title"
                render={({ field }) => (
                  <FormItem>
                    <FormLabel>Job Title</FormLabel>
                    <FormControl>
                      <Input {...field} />
                    </FormControl>
                    <FormMessage />
                  </FormItem>
                )}
              />
              <RHFField
                control={form.control}
                name="manager_id"
                render={({ field }) => (
                  <FormItem>
                    <FormLabel>Manager</FormLabel>
                    <Select
                      value={field.value === null ? NONE_OPTION : String(field.value)}
                      onValueChange={(value) =>
                        field.onChange(value === NONE_OPTION ? null : Number(value))
                      }
                    >
                      <FormControl>
                        <SelectTrigger>
                          <SelectValue placeholder="No manager" />
                        </SelectTrigger>
                      </FormControl>
                      <SelectContent>
                        <SelectItem value={NONE_OPTION}>No manager</SelectItem>
                        {employees
                          .filter((employee) => employee.id !== id)
                          .map((employee) => (
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
                control={form.control}
                name="division_id"
                render={({ field }) => (
                  <FormItem>
                    <FormLabel>Division</FormLabel>
                    <Select
                      value={field.value > 0 ? String(field.value) : undefined}
                      onValueChange={(value) => field.onChange(Number(value))}
                    >
                      <FormControl>
                        <SelectTrigger>
                          <SelectValue placeholder="Select a division" />
                        </SelectTrigger>
                      </FormControl>
                      <SelectContent>
                        {divisions.map((division) => (
                          <SelectItem key={division.id} value={String(division.id)}>
                            {division.name}
                          </SelectItem>
                        ))}
                      </SelectContent>
                    </Select>
                    <FormMessage />
                  </FormItem>
                )}
              />
              <RHFField
                control={form.control}
                name="employment_type"
                render={({ field }) => (
                  <FormItem>
                    <FormLabel>Employment Type</FormLabel>
                    <Select value={field.value} onValueChange={field.onChange}>
                      <FormControl>
                        <SelectTrigger>
                          <SelectValue />
                        </SelectTrigger>
                      </FormControl>
                      <SelectContent>
                        {EMPLOYMENT_TYPES.map((type) => (
                          <SelectItem key={type} value={type}>
                            {type}
                          </SelectItem>
                        ))}
                      </SelectContent>
                    </Select>
                    <FormMessage />
                  </FormItem>
                )}
              />
              <RHFField
                control={form.control}
                name="location_id"
                render={({ field }) => (
                  <FormItem>
                    <FormLabel>Location</FormLabel>
                    <Select
                      value={field.value > 0 ? String(field.value) : undefined}
                      onValueChange={(value) => field.onChange(Number(value))}
                    >
                      <FormControl>
                        <SelectTrigger>
                          <SelectValue placeholder="Select a location" />
                        </SelectTrigger>
                      </FormControl>
                      <SelectContent>
                        {locations.map((location) => (
                          <SelectItem key={location.id} value={String(location.id)}>
                            {location.name}
                          </SelectItem>
                        ))}
                      </SelectContent>
                    </Select>
                    <FormMessage />
                  </FormItem>
                )}
              />
              <RHFField
                control={form.control}
                name="email"
                render={({ field }) => (
                  <FormItem>
                    <FormLabel>Email</FormLabel>
                    <FormControl>
                      <Input type="email" {...field} />
                    </FormControl>
                    <FormMessage />
                  </FormItem>
                )}
              />
              <RHFField
                control={form.control}
                name="phone"
                render={({ field }) => (
                  <FormItem>
                    <FormLabel>Phone</FormLabel>
                    <FormControl>
                      <Input type="tel" {...field} />
                    </FormControl>
                    <FormMessage />
                  </FormItem>
                )}
              />
              {saveMutation.error && <ErrorAlert error={saveMutation.error} />}
              <div className="flex gap-2">
                <Button
                  type="button"
                  variant="secondary"
                  onClick={() => navigate("/organisation")}
                >
                  Cancel
                </Button>
                <Button type="submit" disabled={saveMutation.isPending}>
                  {saveMutation.isPending ? "Saving…" : "Submit"}
                </Button>
              </div>
            </form>
          </Form>
        </CardContent>
      </Card>
    </>
  );
}

type HalfDaySelection = "full" | "first" | "last" | "both";
const HALF_DAY_OPTIONS: { value: HalfDaySelection; label: string }[] = [
  { value: "full", label: "Full days" },
  { value: "first", label: "First day half" },
  { value: "last", label: "Last day half" },
  { value: "both", label: "First and last day half" },
];

function halfDayFlags(selection: HalfDaySelection) {
  return {
    first_day_half: selection === "first" || selection === "both",
    last_day_half: selection === "last" || selection === "both",
  };
}

function leaveDuration(dateFrom: string, dateTo: string, selection: HalfDaySelection) {
  const flags = halfDayFlags(selection);
  const halfDays = Number(flags.first_day_half) + Number(flags.last_day_half);
  return Math.max(
    0,
    workingDaysBetween(dateFrom, dateTo) - halfDays * HALF_DAY_REDUCTION,
  );
}

function halfDaySelection(values: {
  first_day_half: boolean;
  last_day_half: boolean;
}): HalfDaySelection {
  if (values.first_day_half && values.last_day_half) return "both";
  if (values.first_day_half) return "first";
  if (values.last_day_half) return "last";
  return "full";
}

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

const LEAVE_TYPE_FLAGS = [
  ["can_book_in_future", "Can book in future?"],
  ["show_in_others_days", "Show in other days?"],
  ["default_in_others_days", "Default in other days?"],
  ["default_in_leave_request", "Default in leave request?"],
  ["show_in_approve_days", "Show in approve days?"],
] as const;

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

export function LeaveTypesPage() {
  const query = useQuery(leaveTypesQueryOptions());
  return (
    <>
      <PageHeading
        title="Leave Types"
        action={
          <Button asChild>
            <Link to="/leave-types/new">
              <Plus />
              Create Leave Type
            </Link>
          </Button>
        }
      />
      {query.isPending || query.isError ? (
        <QueryMessage error={query.error} />
      ) : (
        <Card>
          <Table>
            <TableHeader>
              <TableRow>
                <TableHead>Name</TableHead>
                <TableHead>Quota</TableHead>
                <TableHead>Colour</TableHead>
                {LEAVE_TYPE_FLAGS.map(([, label]) => (
                  <TableHead key={label}>{label}</TableHead>
                ))}
              </TableRow>
            </TableHeader>
            <TableBody>
              {query.data.map((leaveType) => (
                <TableRow key={leaveType.id}>
                  <TableCell className="font-medium">{leaveType.name}</TableCell>
                  <TableCell>{leaveType.default_quota ?? EMPTY_VALUE}</TableCell>
                  <TableCell>
                    <span className="inline-flex items-center gap-2">
                      <span
                        aria-hidden
                        className="size-4 rounded-sm border"
                        style={{ backgroundColor: leaveType.colour }}
                      />
                      {leaveType.colour}
                    </span>
                  </TableCell>
                  {LEAVE_TYPE_FLAGS.map(([key]) => (
                    <TableCell key={key}>
                      <Checkbox
                        aria-label={`${key} for ${leaveType.name}`}
                        checked={leaveType[key]}
                        onCheckedChange={() => undefined}
                        disabled
                      />
                    </TableCell>
                  ))}
                </TableRow>
              ))}
            </TableBody>
          </Table>
          {query.data.length === 0 && (
            <CardContent>
              <p className="text-sm text-muted-foreground">No leave types configured.</p>
            </CardContent>
          )}
        </Card>
      )}
    </>
  );
}

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
