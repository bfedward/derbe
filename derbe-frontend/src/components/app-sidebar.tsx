import { CalendarDays, Cog, UsersRound } from "lucide-react";
import { Link, useLocation } from "react-router-dom";
import type { CurrentUser } from "@/bindings/CurrentUser";
import { NavMain, type NavMainItem } from "@/components/nav-main";
import { NavUser } from "@/components/nav-user";
import {
  Sidebar,
  SidebarContent,
  SidebarFooter,
  SidebarHeader,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
  SidebarRail,
} from "@/components/ui/sidebar";

export function AppSidebar({ user }: { user: CurrentUser }) {
  const { pathname } = useLocation();
  const canManageEmployees = user.role === "Admin" || user.role === "Manager";
  const isCurrent = (url: string) =>
    pathname === url ||
    (url !== "/dashboard/leave-request" && pathname.startsWith(`${url}/`)) ||
    (url === "/dashboard/leave-request" &&
      /^\/dashboard\/leave-request\/[^/]+$/.test(pathname));
  const navItems: NavMainItem[] = [
    {
      title: "Days Off",
      icon: CalendarDays,
      isActive: [
        "/dashboard/my-leave",
        "/dashboard/team-leave",
        "/dashboard/all-leave",
        "/dashboard/leave-request",
      ].some(isCurrent),
      items: [
        {
          title: "My Leave",
          url: "/dashboard/my-leave",
          isActive: isCurrent("/dashboard/my-leave"),
        },
        ...(canManageEmployees
          ? [
              {
                title: "Team Leave",
                url: "/dashboard/team-leave",
                isActive: isCurrent("/dashboard/team-leave"),
              },
            ]
          : []),
        {
          title: "All Leave",
          url: "/dashboard/all-leave",
          isActive: isCurrent("/dashboard/all-leave"),
        },
        {
          title: "Request Days Off",
          url: "/dashboard/leave-request",
          isActive: isCurrent("/dashboard/leave-request"),
        },
      ],
    },
    {
      title: "Employees",
      icon: UsersRound,
      isActive:
        isCurrent("/dashboard/organisation") ||
        isCurrent("/dashboard/employee/new") ||
        pathname.startsWith("/dashboard/employee/"),
      items: [
        {
          title: "Organisation",
          url: "/dashboard/organisation",
          isActive: isCurrent("/dashboard/organisation"),
        },
        ...(canManageEmployees
          ? [
              {
                title: "Create Employee",
                url: "/dashboard/employee/new",
                isActive: pathname === "/dashboard/employee/new",
              },
            ]
          : []),
      ],
    },
    {
      title: "Lists",
      icon: Cog,
      isActive: [
        "/dashboard/leave-types",
        "/dashboard/locations",
        "/dashboard/divisions",
      ].some(isCurrent),
      items: [
        {
          title: "Leave Types",
          url: "/dashboard/leave-types",
          isActive: isCurrent("/dashboard/leave-types"),
        },
        {
          title: "Locations",
          url: "/dashboard/locations",
          isActive: isCurrent("/dashboard/locations"),
        },
        {
          title: "Divisions",
          url: "/dashboard/divisions",
          isActive: isCurrent("/dashboard/divisions"),
        },
      ],
    },
  ];

  return (
    <Sidebar collapsible="icon">
      <SidebarHeader>
        <SidebarMenu>
          <SidebarMenuItem>
            <SidebarMenuButton asChild size="lg" tooltip="Derbe">
              <Link to="/dashboard/" aria-label="Derbe dashboard">
                <div className="flex size-8 items-center justify-center rounded-lg bg-sidebar-primary text-sidebar-primary-foreground">
                  <span className="text-sm font-bold">d</span>
                </div>
                <div className="grid flex-1 text-left text-sm leading-tight">
                  <span className="truncate font-semibold">derbe</span>
                  <span className="truncate text-xs">Leave management</span>
                </div>
              </Link>
            </SidebarMenuButton>
          </SidebarMenuItem>
        </SidebarMenu>
      </SidebarHeader>
      <SidebarContent>
        <NavMain items={navItems} />
      </SidebarContent>
      <SidebarFooter>
        <NavUser user={user} />
      </SidebarFooter>
      <SidebarRail />
    </Sidebar>
  );
}
