import { useParams } from "react-router-dom";
import { QueryMessage } from "@/components/query-message";
import { EmployeeForm } from "@/features/employees/components/employee-form";

export function EmployeeProfilePage() {
  const { employee_unid } = useParams();
  const employeeUnid = employee_unid;
  return employeeUnid ? (
    <EmployeeForm employee_unid={employeeUnid} />
  ) : (
    <QueryMessage empty="Employee not found." />
  );
}
