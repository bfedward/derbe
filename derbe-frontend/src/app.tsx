import { useQuery } from "@tanstack/react-query";
import {
  BrowserRouter,
  Link,
  Navigate,
  Outlet,
  Route,
  Routes,
  useLocation,
} from "react-router-dom";
import { currentUserQueryOptions } from "@/api";
import { AppSidebar } from "@/components/app-sidebar";
import {
  Breadcrumb,
  BreadcrumbItem,
  BreadcrumbList,
  BreadcrumbPage,
} from "@/components/ui/breadcrumb";
import { Button } from "@/components/ui/button";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Separator } from "@/components/ui/separator";
import { SidebarInset, SidebarProvider, SidebarTrigger } from "@/components/ui/sidebar";
import { ThemeToggle } from "@/components/ui/theme-toggle";
import { ProtectedRoute } from "@/features/auth/components/protected-route";
import { LoginPage } from "@/features/auth/pages/login-page";
import { HomePage } from "@/features/dashboard/pages/home-page";
import { EmployeeFormPage } from "@/features/employees/pages/employee-form-page";
import { EmployeeProfilePage } from "@/features/employees/pages/employee-profile-page";
import { PublicHomePage } from "@/features/home/pages/public-home-page";
import { AllLeavePage } from "@/features/leave/pages/all-leave-page";
import { LeaveRequestFormPage } from "@/features/leave/pages/leave-request-form-page";
import { LeaveRequestProfilePage } from "@/features/leave/pages/leave-request-profile-page";
import { MyLeavePage } from "@/features/leave/pages/my-leave-page";
import { TeamLeavePage } from "@/features/leave/pages/team-leave-page";
import { OrganisationPage } from "@/features/organisation/pages/organisation-page";
import { DivisionsPage } from "@/features/settings/pages/divisions-page";
import { LeaveTypesPage } from "@/features/settings/pages/leave-types-page";
import { LocationsPage } from "@/features/settings/pages/locations-page";
import { NewDivisionPage } from "@/features/settings/pages/new-division-page";
import { NewLeaveTypePage } from "@/features/settings/pages/new-leave-type-page";
import { NewLocationPage } from "@/features/settings/pages/new-location-page";

const PAGE_TITLES: Record<string, string> = {
  "/dashboard": "Dashboard",
  "/dashboard/": "Dashboard",
  "/dashboard/my-leave": "My Leave",
  "/dashboard/all-leave": "All Leave",
  "/dashboard/team-leave": "Team Leave",
  "/dashboard/leave-request": "Request Days Off",
  "/dashboard/organisation": "Organisation",
  "/dashboard/employee/new": "Create Employee",
  "/dashboard/leave-types": "Leave Types",
  "/dashboard/leave-types/new": "New Leave Type",
  "/dashboard/locations": "Locations",
  "/dashboard/locations/new": "New Location",
  "/dashboard/divisions": "Divisions",
  "/dashboard/divisions/new": "New Division",
};

function LoginLayout() {
  return (
    <div className="min-h-screen">
      <header className="border-b bg-card">
        <div className="mx-auto flex max-w-7xl items-center px-4 py-3 sm:px-6">
          <Link to="/" className="text-lg font-semibold tracking-tight">
            derbe
          </Link>
        </div>
      </header>
      <main className="mx-auto w-full max-w-7xl space-y-5 p-4 sm:p-6">
        <Outlet />
      </main>
    </div>
  );
}

function Layout() {
  const { data: currentUser } = useQuery(currentUserQueryOptions());
  const { pathname } = useLocation();
  const pageTitle =
    PAGE_TITLES[pathname] ??
    (pathname.startsWith("/dashboard/leave-request/")
      ? "Leave Request"
      : pathname.startsWith("/dashboard/employee/")
        ? "Employee"
        : "Derbe");

  if (!currentUser) return null;
  if (pathname === "/dashboard") return <Navigate to="/dashboard/" replace />;

  return (
    <SidebarProvider>
      <AppSidebar user={currentUser} />
      <SidebarInset>
        <header className="flex h-14 shrink-0 items-center gap-2 border-b px-4">
          <SidebarTrigger className="-ml-1" />
          <Separator orientation="vertical" className="mr-2 h-4" />
          <Breadcrumb className="flex-1">
            <BreadcrumbList>
              <BreadcrumbItem>
                <BreadcrumbPage>{pageTitle}</BreadcrumbPage>
              </BreadcrumbItem>
            </BreadcrumbList>
          </Breadcrumb>
          <ThemeToggle />
        </header>
        <main className="flex min-w-0 flex-1 flex-col gap-4 p-4 md:p-6">
          <Outlet />
        </main>
      </SidebarInset>
    </SidebarProvider>
  );
}

function NotFoundPage() {
  return (
    <Card>
      <CardHeader>
        <CardTitle>Page not found</CardTitle>
        <CardDescription>Choose a page from the navigation.</CardDescription>
      </CardHeader>
      <CardContent>
        <Button asChild>
          <Link to="/">Go home</Link>
        </Button>
      </CardContent>
    </Card>
  );
}

function LegacyDashboardRedirect() {
  const { pathname, search } = useLocation();
  return <Navigate to={`/dashboard${pathname}${search}`} replace />;
}

function App() {
  return (
    <BrowserRouter>
      <Routes>
        <Route element={<LoginLayout />}>
          <Route path="/login" element={<LoginPage />} />
        </Route>
        <Route path="/" element={<PublicHomePage />} />
        <Route element={<ProtectedRoute />}>
          <Route path="/dashboard" element={<Layout />}>
            <Route index element={<HomePage />} />
            <Route path="my-leave" element={<MyLeavePage />} />
            <Route path="all-leave" element={<AllLeavePage />} />
            <Route path="team-leave" element={<TeamLeavePage />} />
            <Route path="leave-request" element={<LeaveRequestFormPage />} />
            <Route path="leave-request/:id" element={<LeaveRequestProfilePage />} />
            <Route path="organisation" element={<OrganisationPage />} />
            <Route path="employee/new" element={<EmployeeFormPage />} />
            <Route path="employee/:employee_unid" element={<EmployeeProfilePage />} />
            <Route path="leave-types" element={<LeaveTypesPage />} />
            <Route path="leave-types/new" element={<NewLeaveTypePage />} />
            <Route path="locations" element={<LocationsPage />} />
            <Route path="locations/new" element={<NewLocationPage />} />
            <Route path="divisions" element={<DivisionsPage />} />
            <Route path="divisions/new" element={<NewDivisionPage />} />
          </Route>
        </Route>
        <Route path="/my-leave" element={<LegacyDashboardRedirect />} />
        <Route path="/all-leave" element={<LegacyDashboardRedirect />} />
        <Route path="/team-leave" element={<LegacyDashboardRedirect />} />
        <Route path="/leave-request/*" element={<LegacyDashboardRedirect />} />
        <Route path="/organisation" element={<LegacyDashboardRedirect />} />
        <Route path="/employee/*" element={<LegacyDashboardRedirect />} />
        <Route path="/leave-types/*" element={<LegacyDashboardRedirect />} />
        <Route path="/locations/*" element={<LegacyDashboardRedirect />} />
        <Route path="/divisions/*" element={<LegacyDashboardRedirect />} />
        <Route path="*" element={<NotFoundPage />} />
      </Routes>
    </BrowserRouter>
  );
}

export default App;
