import { useQuery } from "@tanstack/react-query";
import {
  BrowserRouter,
  Link,
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
import { ROUTES } from "./routes";

const PAGE_TITLES: Record<string, string> = {
  [ROUTES.home]: "Home",
  [ROUTES.myLeave]: "My Leave",
  [ROUTES.allLeave]: "All Leave",
  [ROUTES.teamLeave]: "Team Leave",
  [ROUTES.newLeaveRequest]: "Request Days Off",
  [ROUTES.organisation]: "Organisation",
  [ROUTES.newEmployee]: "Create Employee",
  [ROUTES.leaveTypes]: "Leave Types",
  [ROUTES.newLeaveType]: "New Leave Type",
  [ROUTES.locations]: "Locations",
  [ROUTES.newLocation]: "New Location",
  [ROUTES.divisions]: "Divisions",
  [ROUTES.newDivision]: "New Division",
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
    (pathname.startsWith("/leave-request/")
      ? "Leave Request"
      : pathname.startsWith("/employee/")
        ? "Employee"
        : "Derbe");

  if (!currentUser) return null;

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

function App() {
  return (
    <BrowserRouter>
      <Routes>
        <Route element={<LoginLayout />}>
          <Route path="/login" element={<LoginPage />} />
        </Route>
        <Route path={ROUTES.publicHome} element={<PublicHomePage />} />
        <Route element={<ProtectedRoute />}>
          <Route element={<Layout />}>
            <Route path={ROUTES.home} element={<HomePage />} />
            <Route path={ROUTES.myLeave} element={<MyLeavePage />} />
            <Route path={ROUTES.allLeave} element={<AllLeavePage />} />
            <Route path={ROUTES.teamLeave} element={<TeamLeavePage />} />
            <Route path={ROUTES.newLeaveRequest} element={<LeaveRequestFormPage />} />
            <Route path="/leave-request/:id" element={<LeaveRequestProfilePage />} />
            <Route path={ROUTES.organisation} element={<OrganisationPage />} />
            <Route path={ROUTES.newEmployee} element={<EmployeeFormPage />} />
            <Route path="/employee/:employee_unid" element={<EmployeeProfilePage />} />
            <Route path={ROUTES.leaveTypes} element={<LeaveTypesPage />} />
            <Route path={ROUTES.newLeaveType} element={<NewLeaveTypePage />} />
            <Route path={ROUTES.locations} element={<LocationsPage />} />
            <Route path={ROUTES.newLocation} element={<NewLocationPage />} />
            <Route path={ROUTES.divisions} element={<DivisionsPage />} />
            <Route path={ROUTES.newDivision} element={<NewDivisionPage />} />
          </Route>
        </Route>
        <Route path="*" element={<NotFoundPage />} />
      </Routes>
    </BrowserRouter>
  );
}

export default App;
