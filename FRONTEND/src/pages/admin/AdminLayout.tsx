import { useState, useEffect } from "react";
import { useNavigate, NavLink, Outlet } from "react-router-dom";
import { useAdmin } from "@/hooks/useAdmin";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { toast } from "sonner";
import { LogOut, FolderKanban, BookOpen, MessageSquare, FileText, User, Loader2 } from "lucide-react";
import { cn } from "@/lib/utils";

export default function AdminLayout() {
  const { user, isAdmin, loading, login, signup, logout } = useAdmin();
  const navigate = useNavigate();

  if (loading) return (
    <div className="flex min-h-screen items-center justify-center">
      <Loader2 className="h-6 w-6 animate-spin text-primary" />
    </div>
  );

  if (!user) return <AuthScreen login={login} signup={signup} />;

  if (!isAdmin) return (
    <div className="container flex min-h-screen flex-col items-center justify-center gap-4 text-center">
      <h1 className="font-grotesk text-3xl font-bold">Not authorized</h1>
      <p className="text-muted-foreground">This account doesn't have admin access.</p>
      <Button variant="outline" onClick={() => { logout(); navigate("/admin"); }}>
        <LogOut className="mr-2 h-4 w-4" /> Sign out
      </Button>
    </div>
  );

  const links = [
    { to: "/admin", label: "About", icon: User, end: true },
    { to: "/admin/projects", label: "Projects", icon: FolderKanban },
    { to: "/admin/blogs", label: "Blog", icon: FileText },
    { to: "/admin/books", label: "Books", icon: BookOpen },
    { to: "/admin/thoughts", label: "Thoughts", icon: MessageSquare },
  ];

  return (
    <div className="min-h-screen">
      <header className="border-b border-border bg-card/50 backdrop-blur">
        <div className="container flex h-14 items-center justify-between">
          <div className="flex items-center gap-2">
            <span className="font-mono text-xs text-primary">/admin</span>
            <span className="font-grotesk font-semibold">Prof Radon</span>
          </div>
          <div className="flex items-center gap-3 text-sm">
            <span className="text-muted-foreground">{user.email}</span>
            <Button size="sm" variant="ghost" onClick={() => { logout(); navigate("/admin"); }}>
              <LogOut className="h-4 w-4" />
            </Button>
          </div>
        </div>
      </header>
      <div className="container grid gap-8 py-8 md:grid-cols-[200px_1fr]">
        <aside>
          <nav className="space-y-1">
            {links.map((l) => (
              <NavLink
                key={l.to}
                to={l.to}
                end={l.end}
                className={({ isActive }) =>
                  cn(
                    "flex items-center gap-2 rounded-md px-3 py-2 text-sm font-medium transition-colors",
                    isActive ? "bg-secondary text-foreground" : "text-muted-foreground hover:bg-secondary/50 hover:text-foreground"
                  )
                }
              >
                <l.icon className="h-4 w-4" />
                {l.label}
              </NavLink>
            ))}
          </nav>
        </aside>
        <main className="min-w-0">
          <Outlet />
        </main>
      </div>
    </div>
  );
}

function LoginScreen({ login }: { login: (email: string, password: string) => Promise<void> }) {
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [busy, setBusy] = useState(false);

  const submit = async (e: React.FormEvent) => {
    e.preventDefault();
    setBusy(true);
    try {
      await login(email, password);
      toast.success("Logged in successfully");
    } catch (err) {
      const message = err instanceof Error ? err.message : "Failed";
      toast.error(message);
    } finally {
      setBusy(false);
    }
  };

  return (
    <div className="flex min-h-screen items-center justify-center px-4">
      <div className="w-full max-w-sm rounded-lg border border-border bg-card p-8 card-shadow">
        <p className="font-mono text-xs uppercase tracking-[0.2em] text-primary">/admin</p>
        <h1 className="mt-2 font-grotesk text-2xl font-bold">Sign in</h1>
        <p className="mt-1 text-sm text-muted-foreground">Restricted area.</p>
        <form onSubmit={submit} className="mt-6 space-y-4">
          <div>
            <Label htmlFor="email">Email</Label>
            <Input id="email" type="email" required value={email} onChange={(e) => setEmail(e.target.value)} className="mt-1.5" />
          </div>
          <div>
            <Label htmlFor="password">Password</Label>
            <Input id="password" type="password" required minLength={8} value={password} onChange={(e) => setPassword(e.target.value)} className="mt-1.5" />
          </div>
          <Button type="submit" disabled={busy} className="w-full">
            {busy ? <Loader2 className="h-4 w-4 animate-spin" /> : "Sign in"}
          </Button>
        </form>
      </div>
    </div>
  );
}

function AuthScreen({ login, signup }: { login: (email: string, password: string) => Promise<void>; signup: (email: string, password: string) => Promise<void> }) {
  const [isSignup, setIsSignup] = useState(false);
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [confirmPassword, setConfirmPassword] = useState("");
  const [busy, setBusy] = useState(false);

  // Check if an account exists during component mount
  useEffect(() => {
    checkIfAccountExists();
  }, []);

  const checkIfAccountExists = async () => {
    // If we already checked and tried to access the page, we know no account exists
    // because we were redirected here from the login check
    setIsSignup(true);
  };

  const submit = async (e: React.FormEvent) => {
    e.preventDefault();
    
    if (isSignup && password !== confirmPassword) {
      toast.error("Passwords don't match");
      return;
    }

    setBusy(true);
    try {
      if (isSignup) {
        await signup(email, password);
        toast.success("Account created successfully");
      } else {
        await login(email, password);
        toast.success("Logged in successfully");
      }
    } catch (err) {
      const message = err instanceof Error ? err.message : "Failed";
      if (message.includes("403") || message.includes("Forbidden")) {
        toast.error("Only profradon@gmail.com can create an account");
      } else if (message.includes("409") || message.includes("Conflict")) {
        toast.error("Account already exists");
        setIsSignup(false);
      } else {
        toast.error(message);
      }
    } finally {
      setBusy(false);
    }
  };

  return (
    <div className="flex min-h-screen items-center justify-center px-4">
      <div className="w-full max-w-sm rounded-lg border border-border bg-card p-8 card-shadow">
        <p className="font-mono text-xs uppercase tracking-[0.2em] text-primary">/admin</p>
        <h1 className="mt-2 font-grotesk text-2xl font-bold">
          {isSignup ? "Create Account" : "Sign in"}
        </h1>
        <p className="mt-1 text-sm text-muted-foreground">
          {isSignup ? "Welcome to the admin area." : "Restricted area."}
        </p>
        
        <form onSubmit={submit} className="mt-6 space-y-4">
          <div>
            <Label htmlFor="email">Email</Label>
            <Input 
              id="email" 
              type="email" 
              required 
              value={email} 
              onChange={(e) => setEmail(e.target.value)}
              placeholder={isSignup ? "profradon@gmail.com" : ""}
              className="mt-1.5" 
            />
            {isSignup && <p className="mt-1 text-xs text-muted-foreground">Only profradon@gmail.com can create an account</p>}
          </div>
          
          <div>
            <Label htmlFor="password">Password</Label>
            <Input 
              id="password" 
              type="password" 
              required 
              minLength={8}
              value={password} 
              onChange={(e) => setPassword(e.target.value)}
              className="mt-1.5" 
            />
          </div>

          {isSignup && (
            <div>
              <Label htmlFor="confirmPassword">Confirm Password</Label>
              <Input 
                id="confirmPassword" 
                type="password" 
                required 
                minLength={8}
                value={confirmPassword} 
                onChange={(e) => setConfirmPassword(e.target.value)}
                className="mt-1.5" 
              />
            </div>
          )}
          
          <Button type="submit" disabled={busy} className="w-full">
            {busy ? <Loader2 className="h-4 w-4 animate-spin" /> : (isSignup ? "Create Account" : "Sign in")}
          </Button>
        </form>

        {!isSignup && (
          <p className="mt-6 text-center text-sm text-muted-foreground">
            Don't have an account?{" "}
            <button
              type="button"
              onClick={() => {
                setIsSignup(true);
                setEmail("");
                setPassword("");
                setConfirmPassword("");
              }}
              className="text-primary hover:underline"
            >
              Create one
            </button>
          </p>
        )}
        
        {isSignup && (
          <p className="mt-6 text-center text-sm text-muted-foreground">
            Already have an account?{" "}
            <button
              type="button"
              onClick={() => {
                setIsSignup(false);
                setEmail("");
                setPassword("");
                setConfirmPassword("");
              }}
              className="text-primary hover:underline"
            >
              Sign in
            </button>
          </p>
        )}
      </div>
    </div>
  );
}
