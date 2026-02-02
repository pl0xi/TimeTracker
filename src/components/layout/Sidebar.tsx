import { NavLink } from "react-router-dom";
import {
  LayoutDashboard,
  BarChart3,
  FolderKanban,
  Tag,
  Settings,
  Clock,
} from "lucide-react";
import { cn } from "../../lib/utils";

const navItems = [
  { to: "/", icon: LayoutDashboard, label: "Dashboard" },
  { to: "/reports", icon: BarChart3, label: "Reports" },
  { to: "/categories", icon: Tag, label: "Categories" },
  { to: "/projects", icon: FolderKanban, label: "Projects" },
  { to: "/settings", icon: Settings, label: "Settings" },
];

export function Sidebar() {
  return (
    <aside className="w-64 border-r border-[hsl(var(--border))] bg-[hsl(var(--card))] flex flex-col">
      <div className="p-4 border-b border-[hsl(var(--border))]">
        <div className="flex items-center gap-2">
          <Clock className="h-6 w-6 text-[hsl(var(--primary))]" />
          <span className="font-semibold text-lg">Time Tracker</span>
        </div>
      </div>

      <nav className="flex-1 p-2">
        <ul className="space-y-1">
          {navItems.map((item) => (
            <li key={item.to}>
              <NavLink
                to={item.to}
                className={({ isActive }) =>
                  cn(
                    "flex items-center gap-3 px-3 py-2 rounded-lg text-sm font-medium transition-colors",
                    isActive
                      ? "bg-[hsl(var(--primary))] text-[hsl(var(--primary-foreground))]"
                      : "text-[hsl(var(--muted-foreground))] hover:bg-[hsl(var(--accent))] hover:text-[hsl(var(--accent-foreground))]"
                  )
                }
              >
                <item.icon className="h-5 w-5" />
                {item.label}
              </NavLink>
            </li>
          ))}
        </ul>
      </nav>

      <div className="p-4 border-t border-[hsl(var(--border))] text-xs text-[hsl(var(--muted-foreground))]">
        Time Tracker v0.1.0
      </div>
    </aside>
  );
}
