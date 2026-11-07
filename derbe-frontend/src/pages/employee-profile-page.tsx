import { useParams } from "react-router-dom";
import { EmployeeForm } from "@/components/forms/employee-form";
import { QueryMessage } from "@/components/query-message";

export function EmployeeProfilePage() {
  const { id } = useParams();
  const employeeId = Number(id);
  return Number.isInteger(employeeId) && employeeId > 0 ? (
    <EmployeeForm id={employeeId} />
  ) : (
    <QueryMessage empty="Employee not found." />
  );
}
