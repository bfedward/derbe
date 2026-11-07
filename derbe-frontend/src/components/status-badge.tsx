import type { LeaveStatus } from "@/bindings/LeaveStatus";
import { Badge } from "@/components/ui/badge";

export function StatusBadge({ status }: { status: LeaveStatus }) {
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
