import { useQuery } from "@tanstack/react-query";
import { Plus } from "lucide-react";
import { Link } from "react-router-dom";
import { leaveTypesQueryOptions } from "@/api";
import { PageHeading } from "@/components/page-heading";
import { QueryMessage } from "@/components/query-message";
import { Button } from "@/components/ui/button";
import { Card, CardContent } from "@/components/ui/card";
import { Checkbox } from "@/components/ui/checkbox";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import { EMPTY_VALUE, LEAVE_TYPE_FLAGS } from "@/constants/constants";

export function LeaveTypesPage() {
  const query = useQuery(leaveTypesQueryOptions());
  return (
    <>
      <PageHeading
        title="Leave Types"
        action={
          <Button asChild>
            <Link to="/leave-types/new">
              <Plus />
              Create Leave Type
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
                <TableHead>Name</TableHead>
                <TableHead>Quota</TableHead>
                <TableHead>Colour</TableHead>
                {LEAVE_TYPE_FLAGS.map(([, label]) => (
                  <TableHead key={label}>{label}</TableHead>
                ))}
              </TableRow>
            </TableHeader>
            <TableBody>
              {query.data.map((leaveType) => (
                <TableRow key={leaveType.id}>
                  <TableCell className="font-medium">{leaveType.name}</TableCell>
                  <TableCell>{leaveType.default_quota ?? EMPTY_VALUE}</TableCell>
                  <TableCell>
                    <span className="inline-flex items-center gap-2">
                      <span
                        aria-hidden
                        className="size-4 rounded-sm border"
                        style={{ backgroundColor: leaveType.colour }}
                      />
                      {leaveType.colour}
                    </span>
                  </TableCell>
                  {LEAVE_TYPE_FLAGS.map(([key]) => (
                    <TableCell key={key}>
                      <Checkbox
                        aria-label={`${key} for ${leaveType.name}`}
                        checked={leaveType[key]}
                        onCheckedChange={() => undefined}
                        disabled
                      />
                    </TableCell>
                  ))}
                </TableRow>
              ))}
            </TableBody>
          </Table>
          {query.data.length === 0 && (
            <CardContent>
              <p className="text-sm text-muted-foreground">No leave types configured.</p>
            </CardContent>
          )}
        </Card>
      )}
    </>
  );
}
