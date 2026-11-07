import { useQuery } from "@tanstack/react-query";
import { Plus } from "lucide-react";
import { Link } from "react-router-dom";
import { locationsQueryOptions } from "@/api";
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

export function LocationsPage() {
  const query = useQuery(locationsQueryOptions());
  return (
    <>
      <PageHeading
        title="Location"
        action={
          <Button asChild>
            <Link to="/locations/new">
              <Plus />
              Create Location
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
              {query.data.map((location) => (
                <TableRow key={location.id}>
                  <TableCell className="font-medium">{location.name}</TableCell>
                </TableRow>
              ))}
            </TableBody>
          </Table>
          {query.data.length === 0 && (
            <CardContent>
              <p className="text-sm text-muted-foreground">No locations configured.</p>
            </CardContent>
          )}
        </Card>
      )}
    </>
  );
}
