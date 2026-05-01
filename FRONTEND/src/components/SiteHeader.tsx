import { Link, NavLink, useLocation } from "react-router-dom";
import { useState } from "react";
import { Menu, X } from "lucide-react";
import { cn } from "@/lib/utils";

const links = [
  { to: "/", label: "Home", end: true },
  { to: "/projects", label: "Projects" },
  { to: "/blog", label: "Blog" },
  { to: "/books", label: "Books" },
  { to: "/thoughts", label: "Thoughts" },
  { to: "/about", label: "About" },
];

export function SiteHeader() {
  const loc = useLocation();
  const [isOpen, setIsOpen] = useState(false);
  
  if (loc.pathname.startsWith("/admin")) return null;

  return (
    <header className="sticky top-0 z-40 border-b border-border/60 bg-background/70 backdrop-blur-xl">
      <div className="container flex h-16 items-center justify-between">
        <Link to="/" className="group flex items-center gap-2">
          <span className="font-mono text-xs text-primary">/&gt;</span>
          <span className="font-grotesk text-lg font-bold tracking-tight">
            Prof <span className="text-gradient">Radon</span>
          </span>
        </Link>
        
        {/* Desktop Navigation */}
        <nav className="hidden sm:flex items-center gap-1 md:gap-2">
          {links.map((l) => (
            <NavLink
              key={l.to}
              to={l.to}
              end={l.end}
              className={({ isActive }) =>
                cn(
                  "rounded-md px-2.5 py-1.5 text-sm font-medium transition-colors hover:text-foreground",
                  isActive ? "text-foreground" : "text-muted-foreground"
                )
              }
            >
              {l.label}
            </NavLink>
          ))}
        </nav>

        {/* Mobile Menu Button */}
        <button
          onClick={() => setIsOpen(!isOpen)}
          className="sm:hidden p-2 hover:bg-secondary rounded-md transition-colors"
        >
          {isOpen ? <X className="h-5 w-5" /> : <Menu className="h-5 w-5" />}
        </button>
      </div>

      {/* Mobile Navigation */}
      {isOpen && (
        <div className="sm:hidden border-t border-border/60 bg-background/95 backdrop-blur-xl">
          <nav className="container flex flex-col gap-1 py-4">
            {links.map((l) => (
              <NavLink
                key={l.to}
                to={l.to}
                end={l.end}
                onClick={() => setIsOpen(false)}
                className={({ isActive }) =>
                  cn(
                    "rounded-md px-2.5 py-2 text-sm font-medium transition-colors hover:text-foreground",
                    isActive ? "text-foreground bg-secondary" : "text-muted-foreground"
                  )
                }
              >
                {l.label}
              </NavLink>
            ))}
          </nav>
        </div>
      )}
    </header>
  );
}
