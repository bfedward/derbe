import { useQuery } from "@tanstack/react-query";
import { Plus } from "lucide-react";
import { Link } from "react-router-dom";
import {
  divisionsQueryOptions,
  employeesQueryOptions,
  locationsQueryOptions,
} from "@/api";
import { PageHeading } from "@/components/page-heading";
import { QueryMessage } from "@/components/query-message";
import { Button } from "@/components/ui/button";
import { Card, CardContent } from "@/components/ui/card";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import { EMPTY_VALUE } from "@/constants/constants";

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
