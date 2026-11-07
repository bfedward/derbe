import type { LeaveStatus } from "@/bindings/LeaveStatus";
import { Badge } from "@/components/ui/badge";

const LEAVE_STATUS_PRESENTATION = {
  Accepted: { label: "Accepted", variant: "accepted" },
  Pending: { label: "Pending", variant: "pending" },
  MoreInfoReq: { label: "More Info Required", variant: "pending" },
  Cancelled: { label: "Cancelled", variant: "rejected" },
  Rejected: { label: "Rejected", variant: "rejected" },
} satisfies Record<
  LeaveStatus,
  { label: string; variant: "accepted" | "pending" | "rejected" }
>;

export function LeaveStatusBadge({ status }: { status: LeaveStatus }) {
  const presentation = LEAVE_STATUS_PRESENTATION[status];

  return <Badge variant={presentation.variant}>{presentation.label}</Badge>;
}
