import { useQuery } from "@tanstack/react-query";
import { Plus } from "lucide-react";
import { Link } from "react-router-dom";
import { divisionsQueryOptions } from "@/api";
import { PageHeading } from "@/components/PageHeading";
import { QueryMessage } from "@/components/QueryMessage";
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

export function DivisionsPage() {
  const query = useQuery(divisionsQueryOptions());
  return (
    <>
      <PageHeading
        title="Division"
        action={
          <Button asChild>
            <Link to="/divisions/new">
              <Plus />
              Create Division
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
              </TableRow>
            </TableHeader>
            <TableBody>
              {query.data.map((division) => (
                <TableRow key={division.id}>
                  <TableCell className="font-medium">{division.name}</TableCell>
                </TableRow>
              ))}
            </TableBody>
          </Table>
          {query.data.length === 0 && (
            <CardContent>
              <p className="text-sm text-muted-foreground">No divisions configured.</p>
            </CardContent>
          )}
        </Card>
      )}
    </>
  );
}
