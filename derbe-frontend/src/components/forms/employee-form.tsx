import { zodResolver } from "@hookform/resolvers/zod";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { useEffect } from "react";
import { useForm } from "react-hook-form";
import { useNavigate } from "react-router-dom";
import { z } from "zod";
import {
  createEmployee,
  DEFAULT_EMPLOYEE_ID,
  divisionsQueryOptions,
  employeeQueryOptions,
  employeesQueryOptions,
  locationsQueryOptions,
  updateEmployee,
} from "@/api";
import { Button } from "@/components/ui/button";
import { Card, CardContent } from "@/components/ui/card";
import {
  Form,
  FormControl,
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
import { EMPLOYMENT_TYPES, NONE_OPTION } from "@/constants/constants";
import { derbeKeys } from "@/query-keys";
import { PageHeading } from "../page-heading";
import { ErrorAlert, QueryMessage } from "../query-message";

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

export function EmployeeForm({ id }: { id?: number }) {
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
