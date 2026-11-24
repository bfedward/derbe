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
    (url !== "/leave-request" && pathname.startsWith(`${url}/`)) ||
    (url === "/leave-request" && /^\/leave-request\/[^/]+$/.test(pathname));
  const navItems: NavMainItem[] = [
    {
      title: "Days Off",
      icon: CalendarDays,
      isActive: ["/my-leave", "/team-leave", "/all-leave", "/leave-request"].some(
        isCurrent,
      ),
      items: [
        { title: "My Leave", url: "/my-leave", isActive: isCurrent("/my-leave") },
        ...(canManageEmployees
          ? [
              {
                title: "Team Leave",
                url: "/team-leave",
                isActive: isCurrent("/team-leave"),
              },
            ]
          : []),
        { title: "All Leave", url: "/all-leave", isActive: isCurrent("/all-leave") },
        {
          title: "Request Days Off",
          url: "/leave-request",
          isActive: isCurrent("/leave-request"),
        },
      ],
    },
    {
      title: "Employees",
      icon: UsersRound,
      isActive:
        isCurrent("/organisation") ||
        isCurrent("/employee/new") ||
        pathname.startsWith("/employee/"),
      items: [
        {
          title: "Organisation",
          url: "/organisation",
          isActive: isCurrent("/organisation"),
        },
        ...(canManageEmployees
          ? [
              {
                title: "Create Employee",
                url: "/employee/new",
                isActive: pathname === "/employee/new",
              },
            ]
          : []),
      ],
    },
    {
      title: "Configuration",
      icon: Cog,
      isActive: ["/leave-types", "/locations", "/divisions"].some(isCurrent),
      items: [
        {
          title: "Leave Types",
          url: "/leave-types",
          isActive: isCurrent("/leave-types"),
        },
        { title: "Locations", url: "/locations", isActive: isCurrent("/locations") },
        { title: "Divisions", url: "/divisions", isActive: isCurrent("/divisions") },
      ],
    },
  ];

  return (
    <Sidebar collapsible="icon">
      <SidebarHeader>
        <SidebarMenu>
          <SidebarMenuItem>
            <SidebarMenuButton asChild size="lg" tooltip="Derbe">
              <Link to="/" aria-label="Derbe home">
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
