import { useQuery } from "@tanstack/react-query";
import { Navigate, Outlet } from "react-router-dom";

import { ApiError, currentUserQueryOptions } from "@/api";

export function ProtectedRoute() {
  const { isPending, isError, error, refetch } = useQuery(currentUserQueryOptions());

  // The session check is still in progress.
  if (isPending) {
    return <p>Checking your session...</p>;
  }

  // No valid session: send the user to the login page.
  if (isError && error instanceof ApiError && error.status === 401) {
    return <Navigate to="/login" replace />;
  }

  // Something other than an authentication failure went wrong.
  if (isError) {
    return (
      <div>
        <p>Unable to verify your session. Please try again.</p>
        <button type="button" onClick={() => refetch()}>
          Retry
        </button>
      </div>
    );
  }

  // The session is valid.
  return <Outlet />;
}
