import { QueryClient } from "@tanstack/react-query";
import { ApiError } from "@/api";

const QUERY_STALE_TIME_MS = 30_000;
const MAX_RETRIES = 3;

export const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      staleTime: QUERY_STALE_TIME_MS,
      refetchOnWindowFocus: false,
      // A 4xx (forbidden, not found, invalid) gets the same answer every time,
      // so only retry server errors and network failures.
      retry: (failureCount, error) => {
        if (error instanceof ApiError && error.status < 500) return false;
        return failureCount < MAX_RETRIES;
      },
    },
  },
});
