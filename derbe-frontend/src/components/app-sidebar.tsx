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
import { ROUTES } from "@/routes";

export function AppSidebar({ user }: { user: CurrentUser }) {
  const { pathname } = useLocation();
  const canManageEmployees = user.role === "Admin" || user.role === "Manager";
  const isCurrent = (url: string) => pathname === url || pathname.startsWith(`${url}/`);
  const navItems: NavMainItem[] = [
    {
      title: "Days Off",
      icon: CalendarDays,
      isActive: [
        ROUTES.myLeave,
        ROUTES.teamLeave,
        ROUTES.allLeave,
        ROUTES.newLeaveRequest,
      ].some(isCurrent),
      items: [
        {
          title: "My Leave",
          url: ROUTES.myLeave,
          isActive: isCurrent(ROUTES.myLeave),
        },
        ...(canManageEmployees
          ? [
              {
                title: "Team Leave",
                url: ROUTES.teamLeave,
                isActive: isCurrent(ROUTES.teamLeave),
              },
            ]
          : []),
        {
          title: "All Leave",
          url: ROUTES.allLeave,
          isActive: isCurrent(ROUTES.allLeave),
        },
        {
          title: "Request Days Off",
          url: ROUTES.newLeaveRequest,
          isActive: isCurrent(ROUTES.newLeaveRequest),
        },
      ],
    },
    {
      title: "Employees",
      icon: UsersRound,
      isActive:
        isCurrent(ROUTES.organisation) ||
        isCurrent(ROUTES.newEmployee) ||
        pathname.startsWith("/employee/"),
      items: [
        {
          title: "Organisation",
          url: ROUTES.organisation,
          isActive: isCurrent(ROUTES.organisation),
        },
        ...(canManageEmployees
          ? [
              {
                title: "Create Employee",
                url: ROUTES.newEmployee,
                isActive: pathname === ROUTES.newEmployee,
              },
            ]
          : []),
      ],
    },
    {
      title: "Lists",
      icon: Cog,
      isActive: [ROUTES.leaveTypes, ROUTES.locations, ROUTES.divisions].some(isCurrent),
      items: [
        {
          title: "Leave Types",
          url: ROUTES.leaveTypes,
          isActive: isCurrent(ROUTES.leaveTypes),
        },
        {
          title: "Locations",
          url: ROUTES.locations,
          isActive: isCurrent(ROUTES.locations),
        },
        {
          title: "Divisions",
          url: ROUTES.divisions,
          isActive: isCurrent(ROUTES.divisions),
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
              <Link to={ROUTES.home} aria-label="Derbe home">
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
