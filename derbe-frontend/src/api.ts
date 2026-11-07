import type { AllLeaveRequest } from "@/bindings/AllLeaveRequest";
import type { AllLeaveResponse } from "@/bindings/AllLeaveResponse";
import type { DivisionResponse } from "@/bindings/DivisionResponse";
import type { EmployeeCreate } from "@/bindings/EmployeeCreate";
import type { EmployeeDetailsResponse } from "@/bindings/EmployeeDetailsResponse";
import type { EmployeeResponse } from "@/bindings/EmployeeResponse";
import type { EmployeeUpdate } from "@/bindings/EmployeeUpdate";
import type { LeaveRequestCreate } from "@/bindings/LeaveRequestCreate";
import type { LeaveRequestResponse } from "@/bindings/LeaveRequestResponse";
import type { LeaveRequestStatusUpdate } from "@/bindings/LeaveRequestStatusUpdate";
import type { LeaveRequestUpdate } from "@/bindings/LeaveRequestUpdate";
import type { LeaveStatus } from "@/bindings/LeaveStatus";
import type { LeaveTypeCreate } from "@/bindings/LeaveTypeCreate";
import type { LeaveTypeResponse } from "@/bindings/LeaveTypeResponse";
import type { LocationResponse } from "@/bindings/LocationResponse";
import type { MyLeaveRequest } from "@/bindings/MyLeaveRequest";
import type { MyLeaveResponse } from "@/bindings/MyLeaveResponse";
import { dateRangeIso } from "@/lib/dates";
import { derbeKeys } from "@/query-keys";

const API_DEFAULT_BASE_URL = "http://127.0.0.1:8001";
const API_BASE_URL = (import.meta.env.VITE_API_BASE_URL ?? API_DEFAULT_BASE_URL).replace(
  /\/$/,
  "",
);
const API_PATHS = {
  divisions: "/divisions",
  employees: "/employees",
  employeeDetails: "/employees/detailsTable",
  leaveRequests: "/leaveRequests",
  myLeave: "/myLeave",
  allLeave: "/allLeave",
  leaveTypes: "/leaveTypes",
  locations: "/locations",
} as const;

export const DEFAULT_EMPLOYEE_ID = 1;
export const DEFAULT_LEAVE_TYPE_IDS = [1, 5] as const;
export const DEFAULT_LEAVE_STATUSES: LeaveStatus[] = ["Pending"];

export class ApiError extends Error {
  constructor(
    message: string,
    readonly status: number,
  ) {
    super(message);
    this.name = "ApiError";
  }
}

async function request<T>(path: string, init?: RequestInit): Promise<T> {
  const headers = new Headers(init?.headers);
  headers.set("Accept", "application/json");
  if (init?.body && !headers.has("Content-Type"))
    headers.set("Content-Type", "application/json");
  const response = await fetch(`${API_BASE_URL}${path}`, {
    ...init,
    headers,
  });

  if (!response.ok) {
    const body = await response.text();
    throw new ApiError(
      body || `${response.status} ${response.statusText}`,
      response.status,
    );
  }

  if (response.status === 204) return undefined as T;
  return (await response.json()) as T;
}

function postJson<T>(path: string, body: unknown, signal?: AbortSignal): Promise<T> {
  return request<T>(path, { method: "POST", body: JSON.stringify(body), signal });
}

function putJson<T>(path: string, body: unknown, signal?: AbortSignal): Promise<T> {
  return request<T>(path, { method: "PUT", body: JSON.stringify(body), signal });
}

export function getEmployees(signal?: AbortSignal) {
  return request<EmployeeResponse[]>(API_PATHS.employees, { signal });
}

export function getEmployeeDetails(signal?: AbortSignal) {
  return request<EmployeeDetailsResponse[]>(API_PATHS.employeeDetails, { signal });
}

export function getEmployee(id: number, signal?: AbortSignal) {
  return request<EmployeeResponse>(`${API_PATHS.employees}/${id}`, { signal });
}

export function createEmployee(input: EmployeeCreate) {
  return postJson<EmployeeResponse>(API_PATHS.employees, input);
}

export function updateEmployee(id: number, input: EmployeeUpdate) {
  return putJson<EmployeeResponse>(`${API_PATHS.employees}/${id}`, input);
}

export function getDivisions(signal?: AbortSignal) {
  return request<DivisionResponse[]>(API_PATHS.divisions, { signal });
}

export function createDivision(input: DivisionCreate) {
  return postJson<DivisionResponse>(API_PATHS.divisions, input);
}

export function getLocations(signal?: AbortSignal) {
  return request<LocationResponse[]>(API_PATHS.locations, { signal });
}

export function createLocation(input: LocationCreate) {
  return postJson<LocationResponse>(API_PATHS.locations, input);
}

export function getLeaveTypes(signal?: AbortSignal) {
  return request<LeaveTypeResponse[]>(API_PATHS.leaveTypes, { signal });
}

export function getLeaveType(id: number, signal?: AbortSignal) {
  return request<LeaveTypeResponse>(`${API_PATHS.leaveTypes}/${id}`, { signal });
}

export function createLeaveType(input: LeaveTypeCreate) {
  return postJson<LeaveTypeResponse>(API_PATHS.leaveTypes, input);
}

export function getMyLeave(year: number, signal?: AbortSignal) {
  const input: MyLeaveRequest = {
    employee_id: DEFAULT_EMPLOYEE_ID,
    leave_type_ids: [...DEFAULT_LEAVE_TYPE_IDS],
    leave_statuses: DEFAULT_LEAVE_STATUSES,
    date_from: `${year}-01-01`,
    date_to: `${year}-12-31`,
  };
  return postJson<MyLeaveResponse[]>(API_PATHS.myLeave, input, signal);
}

export function getAllLeave(input: AllLeaveRequest, signal?: AbortSignal) {
  return postJson<AllLeaveResponse[]>(API_PATHS.allLeave, input, signal);
}

export function getLeaveRequest(id: number, signal?: AbortSignal) {
  return request<LeaveRequestResponse>(`${API_PATHS.leaveRequests}/${id}`, { signal });
}

export function createLeaveRequest(input: LeaveRequestCreate) {
  return postJson<LeaveRequestResponse>(API_PATHS.leaveRequests, input);
}

export function updateLeaveRequest(id: number, input: LeaveRequestUpdate) {
  return putJson<LeaveRequestResponse>(`${API_PATHS.leaveRequests}/${id}`, input);
}

export function updateLeaveRequestStatus(id: number, input: LeaveRequestStatusUpdate) {
  return putJson<LeaveRequestResponse>(`${API_PATHS.leaveRequests}/${id}/status`, input);
}

import { queryOptions } from "@tanstack/react-query";
import type { DivisionCreate } from "./bindings/DivisionCreate";
import type { LocationCreate } from "./bindings/LocationCreate";

export const employeesQueryOptions = () =>
  queryOptions({
    queryKey: derbeKeys.employees(),
    queryFn: ({ signal }) => getEmployees(signal),
  });

export const employeeQueryOptions = (id: number) =>
  queryOptions({
    queryKey: derbeKeys.employee(id),
    queryFn: ({ signal }) => getEmployee(id, signal),
  });

export const employeeDetailsQueryOptions = () =>
  queryOptions({
    queryKey: derbeKeys.employeeDetails(),
    queryFn: ({ signal }) => getEmployeeDetails(signal),
  });

export const divisionsQueryOptions = () =>
  queryOptions({
    queryKey: derbeKeys.divisions(),
    queryFn: ({ signal }) => getDivisions(signal),
  });

export const locationsQueryOptions = () =>
  queryOptions({
    queryKey: derbeKeys.locations(),
    queryFn: ({ signal }) => getLocations(signal),
  });

export const leaveTypesQueryOptions = () =>
  queryOptions({
    queryKey: derbeKeys.leaveTypes(),
    queryFn: ({ signal }) => getLeaveTypes(signal),
  });

export const leaveTypeQueryOptions = (id: number) =>
  queryOptions({
    queryKey: derbeKeys.leaveType(id),
    queryFn: ({ signal }) => getLeaveType(id, signal),
  });

export const myLeaveQueryOptions = (year: number) =>
  queryOptions({
    queryKey: derbeKeys.myLeave(year),
    queryFn: ({ signal }) => getMyLeave(year, signal),
  });

export const allLeaveQueryOptions = (year: number, month: number) => {
  const dateRange = dateRangeIso(year, month);
  return queryOptions({
    queryKey: derbeKeys.allLeave(year, month),
    queryFn: ({ signal }) => getAllLeave(dateRange, signal),
  });
};

export const leaveRequestQueryOptions = (id: number) =>
  queryOptions({
    queryKey: derbeKeys.leaveRequest(id),
    queryFn: ({ signal }) => getLeaveRequest(id, signal),
  });
