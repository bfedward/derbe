import { Alert, AlertDescription, AlertTitle } from "@/components/ui/alert";
import { Skeleton } from "@/components/ui/skeleton";

export function ErrorAlert({ error }: { error: Error }) {
  return (
    <Alert variant="destructive">
      <AlertTitle>Something went wrong</AlertTitle>
      <AlertDescription>{error.message}</AlertDescription>
    </Alert>
  );
}

export function QueryMessage({ error, empty }: { error?: Error | null; empty?: string }) {
  if (error) return <ErrorAlert error={error} />;

  if (empty) {
    return <p className="text-sm text-muted-foreground">{empty}</p>;
  }

  return (
    <output aria-label="Loading" className="grid gap-3">
      <Skeleton className="h-8 w-48" />
      <Skeleton className="h-36 w-full" />
    </output>
  );
}
