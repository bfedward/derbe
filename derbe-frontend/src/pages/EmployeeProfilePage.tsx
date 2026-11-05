import { useParams } from "react-router-dom";
import { EmployeeForm } from "@/components/forms/EmployeeForm";
import { QueryMessage } from "@/components/QueryMessage";

export function EmployeeProfilePage() {
  const { id } = useParams();
  const employeeId = Number(id);
  return Number.isInteger(employeeId) && employeeId > 0 ? (
    <EmployeeForm id={employeeId} />
  ) : (
    <QueryMessage empty="Employee not found." />
  );
}
