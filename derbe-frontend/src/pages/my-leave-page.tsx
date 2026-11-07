import { useQuery } from "@tanstack/react-query";
import { Pencil, Plus } from "lucide-react";
import { Link } from "react-router-dom";
import { myLeaveQueryOptions } from "@/api";
import { PageHeading } from "@/components/page-heading";
import { QueryMessage } from "@/components/query-message";
import { LeaveStatusBadge } from "@/components/leave-status-badge";
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
import { formatDate } from "@/lib/dates";

export function MyLeavePage() {
  const year = new Date().getFullYear();
  const query = useQuery(myLeaveQueryOptions(year));
  return (
    <>
      <PageHeading
        title="My Leave"
        action={
          <Button asChild>
            <Link to="/leave-request">
              <Plus />
              Request Days Off
            </Link>
          </Button>
        }
      />
      {query.isPending || query.isError ? (
        <QueryMessage error={query.error} />
      ) : (
        <Card>
          <Table>
            <TableHeader>
              <TableRow>
                <TableHead>Leave Type</TableHead>
                <TableHead>Status</TableHead>
                <TableHead>Date From</TableHead>
                <TableHead>Date To</TableHead>
                <TableHead>Total Duration</TableHead>
                <TableHead />
              </TableRow>
            </TableHeader>
            <TableBody>
              {query.data.map((leave) => (
                <TableRow key={leave.leave_request_id}>
                  <TableCell className="font-medium">{leave.leave_type}</TableCell>
                  <TableCell>
                    <LeaveStatusBadge status={leave.leave_status} />
                  </TableCell>
                  <TableCell>{formatDate(leave.date_from)}</TableCell>
                  <TableCell>{formatDate(leave.date_to)}</TableCell>
                  <TableCell>{leave.total_duration}</TableCell>
                  <TableCell>
                    <Button
                      size="icon"
                      variant="ghost"
                      aria-label="Edit leave request"
                      asChild
                    >
                      <Link to={`/leave-request/${leave.leave_request_id}`}>
                        <Pencil />
                      </Link>
                    </Button>
                  </TableCell>
                </TableRow>
              ))}
            </TableBody>
          </Table>
          {query.data.length === 0 && (
            <CardContent>
              <p className="text-sm text-muted-foreground">
                No leave requests for this year.
              </p>
            </CardContent>
          )}
        </Card>
      )}
    </>
  );
}
