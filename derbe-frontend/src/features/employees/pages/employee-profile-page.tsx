import { useParams } from "react-router-dom";
import { QueryMessage } from "@/components/query-message";
import { EmployeeForm } from "@/features/employees/components/employee-form";

export function EmployeeProfilePage() {
  const { employee_unid } = useParams<{ employee_unid: string }>();
  return employee_unid ? (
    <EmployeeForm employee_unid={employee_unid} />
  ) : (
    <QueryMessage empty="Employee not found." />
  );
}
