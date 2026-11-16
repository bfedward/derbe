import { useParams } from "react-router-dom";
import { EmployeeForm } from "@/components/forms/employee-form";
import { QueryMessage } from "@/components/query-message";

export function EmployeeProfilePage() {
  const { employee_unid } = useParams();
  const employeeUnid = employee_unid;
  return employeeUnid ? (
    <EmployeeForm employee_unid={employeeUnid} />
  ) : (
    <QueryMessage empty="Employee not found." />
  );
}
