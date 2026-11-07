import { CalendarDays, ChevronDown, Settings2, Users } from "lucide-react";
import {
  BrowserRouter,
  Link,
  Outlet,
  Route,
  Routes,
  useLocation,
} from "react-router-dom";
import { Button } from "@/components/ui/button";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import { ThemeToggle } from "@/components/ui/theme-toggle";
import {
  AllLeavePage,
  EmployeeFormPage,
  EmployeeProfilePage,
  HomePage,
  LeaveRequestFormPage,
  LeaveRequestProfilePage,
  LeaveTypesPage,
  MyLeavePage,
  NewLeaveTypePage,
  OrganisationPage,
} from "@/pages";
import { DivisionsPage } from "./pages/DivisionsPage";
import { LocationsPage } from "./pages/LocationsPage";
import { NewDivisionPage } from "./pages/NewDivisionPage";
import { NewLocationPage } from "./pages/NewLocationPage";

function NavigationMenu({
  title,
  icon,
  items,
}: {
  title: string;
  icon: React.ReactNode;
  items: Array<{ label: string; to: string }>;
}) {
  const location = useLocation();
  const isActive = items.some(
    ({ to }) => location.pathname === to || location.pathname.startsWith(`${to}/`),
  );

  return (
    <DropdownMenu>
      <DropdownMenuTrigger asChild>
        <Button variant={isActive ? "secondary" : "ghost"} className="group gap-2">
          {icon}
          {title}
          <ChevronDown className="size-4 text-muted-foreground transition-transform group-data-[state=open]:rotate-180" />
        </Button>
      </DropdownMenuTrigger>
      <DropdownMenuContent align="start" className="min-w-48">
        {items.map(({ label, to }) => (
          <DropdownMenuItem key={to} asChild>
            <Link to={to} className="cursor-pointer">
              {label}
            </Link>
          </DropdownMenuItem>
        ))}
      </DropdownMenuContent>
    </DropdownMenu>
  );
}

function Layout() {
  return (
    <div className="min-h-screen">
      <header className="border-b bg-card">
        <div className="mx-auto flex max-w-7xl flex-wrap items-center justify-between gap-3 px-4 py-3 sm:px-6">
          <Link to="/" className="text-lg font-semibold tracking-tight">
            derbe
          </Link>
          <nav className="flex flex-wrap items-center gap-1">
            <NavigationMenu
              title="Days Off"
              icon={<CalendarDays />}
              items={[
                { label: "My Leave", to: "/my-leave" },
                { label: "All Leave", to: "/all-leave" },
                { label: "Request Days Off", to: "/leave-request" },
              ]}
            />
            <NavigationMenu
              title="Employees"
              icon={<Users />}
              items={[
                { label: "Organisation", to: "/organisation" },
                { label: "Create Employee", to: "/employee/new" },
              ]}
            />
            <NavigationMenu
              title="Configuration"
              icon={<Settings2 />}
              items={[
                { label: "Leave Types", to: "/leave-types" },
                { label: "Locations", to: "/locations" },
                { label: "Divisions", to: "/divisions" },
              ]}
            />
            <ThemeToggle />
          </nav>
        </div>
      </header>
      <main className="mx-auto w-full max-w-7xl space-y-5 p-4 sm:p-6">
        <Outlet />
      </main>
    </div>
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
        <Route element={<Layout />}>
          <Route index element={<HomePage />} />
          <Route path="my-leave" element={<MyLeavePage />} />
          <Route path="all-leave" element={<AllLeavePage />} />
          <Route path="leave-request" element={<LeaveRequestFormPage />} />
          <Route path="leave-request/:id" element={<LeaveRequestProfilePage />} />
          <Route path="organisation" element={<OrganisationPage />} />
          <Route path="employee/new" element={<EmployeeFormPage />} />
          <Route path="employee/:id" element={<EmployeeProfilePage />} />
          <Route path="leave-types" element={<LeaveTypesPage />} />
          <Route path="leave-types/new" element={<NewLeaveTypePage />} />
          <Route path="locations" element={<LocationsPage />} />
          <Route path="locations/new" element={<NewLocationPage />} />
          <Route path="divisions" element={<DivisionsPage />} />
          <Route path="divisions/new" element={<NewDivisionPage />} />
          <Route path="*" element={<NotFoundPage />} />
        </Route>
      </Routes>
    </BrowserRouter>
  );
}

export default App;
