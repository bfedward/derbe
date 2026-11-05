import { Link } from "react-router-dom";
import { PageHeading } from "@/components/PageHeading";
import { Button } from "@/components/ui/button";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";

export function HomePage() {
  return (
    <>
      <PageHeading
        title="Derbe"
        description="Manage leave, employees, and leave types."
      />
      <div className="grid gap-4 sm:grid-cols-3">
        {[
          ["My Leave", "/my-leave", "View requests and request days off."],
          ["All Leave", "/all-leave", "See the team leave calendar."],
          ["Organisation", "/organisation", "Manage employee records."],
        ].map(([title, href, description]) => (
          <Card key={href} className="transition-colors hover:border-primary/40">
            <CardHeader>
              <CardTitle>{title}</CardTitle>
              <CardDescription>{description}</CardDescription>
            </CardHeader>
            <CardContent>
              <Button variant="outline" asChild>
                <Link to={href}>Open</Link>
              </Button>
            </CardContent>
          </Card>
        ))}
      </div>
    </>
  );
}
