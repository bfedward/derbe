import { Link } from "react-router-dom";
import type { EmployeeResponse } from "@/bindings/EmployeeResponse";
import {
  type OrganisationNode,
  useOrganisationChart,
} from "@/components/my-components/use-organisation-chart";
import { Avatar, AvatarFallback } from "@/components/ui/avatar";
import { Badge } from "@/components/ui/badge";
import { Card, CardContent } from "@/components/ui/card";
import { ROLE_LABELS } from "@/features/employees/constants";
import { ROUTES } from "@/routes";

type OrganisationChartProps = {
  employees: EmployeeResponse[];
  canViewEmployees: boolean;
};

function PersonCard({
  employee,
  canViewEmployees,
  reportCount,
}: {
  employee: EmployeeResponse;
  canViewEmployees: boolean;
  reportCount: number;
}) {
  const name = `${employee.first_name} ${employee.last_name}`;
  const initials = `${employee.first_name[0] ?? ""}${employee.last_name[0] ?? ""}`;
  return (
    <Card className="w-64 border-t-2 border-t-primary/60 shadow-sm">
      <CardContent className="flex-row items-center gap-3">
        <Avatar className="size-10">
          <AvatarFallback>{initials}</AvatarFallback>
        </Avatar>
        <div className="min-w-0 flex-1">
          <div className="truncate text-sm font-semibold">
            {canViewEmployees ? (
              <Link
                className="text-foreground hover:text-primary hover:underline"
                to={ROUTES.employee(employee.employee_unid)}
              >
                {name}
              </Link>
            ) : (
              name
            )}
          </div>
          <p className="truncate text-xs text-muted-foreground">
            {employee.job_title || "No job title"}
          </p>
        </div>
      </CardContent>
      <div className="flex items-center justify-between border-t px-4 py-3">
        <Badge variant="secondary">{ROLE_LABELS[employee.role]}</Badge>
        {reportCount > 0 && (
          <span className="text-xs text-muted-foreground">
            {reportCount} {reportCount === 1 ? "report" : "reports"}
          </span>
        )}
      </div>
    </Card>
  );
}

function ChartNode({
  node,
  canViewEmployees,
  isChild = false,
}: {
  node: OrganisationNode;
  canViewEmployees: boolean;
  isChild?: boolean;
}) {
  return (
    <li
      className={`flex flex-col items-center ${isChild ? "relative before:absolute before:-top-6 before:left-1/2 before:h-6 before:border-l before:border-border before:content-['']" : ""}`}
    >
      <PersonCard
        employee={node.employee}
        canViewEmployees={canViewEmployees}
        reportCount={node.reports.length}
      />
      {node.reports.length > 0 && (
        <>
          <div aria-hidden="true" className="h-6 border-l border-border" />
          <ul className="flex items-start justify-center gap-6 border-t border-border pt-6">
            {node.reports.map((report) => (
              <ChartNode
                key={report.employee.employee_unid}
                node={report}
                canViewEmployees={canViewEmployees}
                isChild
              />
            ))}
          </ul>
        </>
      )}
    </li>
  );
}

export function OrganisationChart({
  employees,
  canViewEmployees,
}: OrganisationChartProps) {
  const roots = useOrganisationChart(employees);
  if (employees.length === 0) {
    return (
      <Card>
        <CardContent className="py-8 text-sm text-muted-foreground">
          No employees yet.
        </CardContent>
      </Card>
    );
  }

  return (
    <Card>
      <div className="overflow-x-auto">
        <ul className="flex min-w-max items-start justify-center gap-10 px-8 py-8">
          {roots.map((node) => (
            <ChartNode
              key={node.employee.employee_unid}
              node={node}
              canViewEmployees={canViewEmployees}
            />
          ))}
        </ul>
      </div>
    </Card>
  );
}
