import { useEffect, useState } from "react";
import { Link } from "react-router-dom";
import { api } from "@/integrations/client";
import { ArrowRight, Github, Mail, Heart } from "lucide-react";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { RichText } from "@/components/RichText";
import { toast } from "sonner";
import type { Project, Blog, Thought } from "@/integrations/types";

export default function Index() {
  const [projects, setProjects] = useState<Project[]>([]);
  const [blogs, setBlogs] = useState<Blog[]>([]);
  const [thought, setThought] = useState<Thought | null>(null);
  const [displayedText, setDisplayedText] = useState("");
  const [email, setEmail] = useState("");
  const [subscribing, setSubscribing] = useState(false);

  const fullText = `Hi, I'm Prof Radon.
I build, write, and think out loud. </>`;
  useEffect(() => {
    document.title = "Prof Radon — Builder, writer, learner";
    const meta = document.querySelector('meta[name="description"]') || document.head.appendChild(Object.assign(document.createElement("meta"), { name: "description" }));
    meta.setAttribute("content", "The personal site of Prof Radon — projects, blog posts, books, and notes.");

    (async () => {
      const [p, b, t] = await Promise.all([
        api.get<Project[]>("/api/projects?limit=3"),
        api.get<Blog[]>("/api/blogs?limit=3"),
        api.get<Thought[]>("/api/thoughts?limit=1").then(({ data }) => data?.[0] || null),
      ]);
      if (p.data) setProjects(p.data);
      if (b.data) setBlogs(b.data);
      setThought(t);
    })();
  }, [fullText]);

  useEffect(() => {
    if (displayedText.length < fullText.length) {
      const timer = setTimeout(() => {
        setDisplayedText(fullText.slice(0, displayedText.length + 1));
      }, 50);
      return () => clearTimeout(timer);
    }
  }, [displayedText, fullText]);

  const handleSubscribe = async () => {
    if (!email) {
      toast.error("Please enter your email");
      return;
    }
    setSubscribing(true);
    try {
      toast.success("Thanks for subscribing!");
      setEmail("");
    } catch (e) {
      toast.error("Failed to subscribe");
    } finally {
      setSubscribing(false);
    }
  };

  return (
    <div>
      {/* Hero */}
      <section className="relative overflow-hidden">
        <div className="absolute inset-0 grid-bg opacity-40" aria-hidden />
        <div className="container relative py-16 sm:py-24 md:py-32">
          <p className="font-mono text-xs uppercase tracking-[0.25em] text-primary animate-fade-in">// portfolio</p>
          <h1 className="mt-4 max-w-4xl font-grotesk text-4xl font-bold tracking-tight sm:text-5xl md:text-7xl animate-fade-up whitespace-pre-wrap">
            <span className="text-gradient">{displayedText}</span>
          </h1>
          <p className="mt-6 max-w-2xl text-base sm:text-lg text-muted-foreground animate-fade-up">
            A quiet corner of the internet for my projects, essays, the books I'm reading, and the thoughts I can't shake.
          </p>
          <div className="mt-8 flex flex-wrap items-center gap-6 animate-fade-up">
            <img
              src="/profile.jpg"
              alt="Profile photo"
              className="h-28 w-28 rounded-full border border-border object-cover"
            />
            <div className="max-w-2xl">
              <p className="font-mono text-xs uppercase tracking-[0.25em] text-primary">Profile photo</p>
            </div>
          </div>
          <div className="mt-8 flex flex-wrap gap-3 animate-fade-up">
            <Button asChild size="lg" className="glow-shadow">
              <Link to="/projects">See my work <ArrowRight className="ml-1 h-4 w-4" /></Link>
            </Button>
            <Button asChild size="lg" variant="outline">
              <Link to="/blog">Read the blog</Link>
            </Button>
            <Button asChild size="lg" variant="ghost">
              <a href="https://github.com/rustyRadon" target="_blank" rel="noreferrer"><Github className="mr-1 h-4 w-4" /> GitHub</a>
            </Button>
          </div>
        </div>
      </section>

      {/* Latest thought */}
      {thought && (
        <section className="container py-12">
          <div className="rounded-lg border border-border bg-card p-6 sm:p-8 card-shadow">
            <p className="font-mono text-xs uppercase tracking-[0.2em] text-primary">Latest thought</p>
            <div className="mt-3 font-lora text-lg sm:text-xl text-foreground/90">
              <RichText html={thought.content} />
            </div>
          </div>
        </section>
      )}

      {/* Featured projects */}
      <section className="container py-12 sm:py-16">
        <div className="mb-8 flex items-end justify-between">
          <h2 className="font-grotesk text-2xl sm:text-3xl md:text-4xl font-bold">Selected work</h2>
          <Link to="/projects" className="text-xs sm:text-sm text-primary hover:underline">All projects →</Link>
        </div>
        <div className="grid gap-6 sm:grid-cols-2 lg:grid-cols-3">
          {projects.length === 0 && <p className="text-muted-foreground">No projects yet — add some from the admin panel.</p>}
          {projects.map((p) => (
            <article key={p.id} className="group rounded-lg border border-border bg-card p-6 card-shadow transition-all hover:-translate-y-1 hover:border-primary/40">
              <h3 className="font-grotesk text-xl font-semibold">{p.title}</h3>
              <p className="mt-2 line-clamp-3 text-sm text-muted-foreground">{p.description}</p>
              <div className="mt-4 flex flex-wrap gap-1.5">
                {p.tags?.slice(0, 4).map((t) => (
                  <span key={t} className="rounded-full bg-secondary px-2 py-0.5 font-mono text-xs text-muted-foreground">{t}</span>
                ))}
              </div>
              {p.url && (
                <a href={p.url} target="_blank" rel="noreferrer" className="mt-4 inline-flex items-center gap-1 text-sm text-primary opacity-0 transition-opacity group-hover:opacity-100">
                  Visit <ArrowRight className="h-3 w-3" />
                </a>
              )}
            </article>
          ))}
        </div>
      </section>

      {/* Recent posts */}
      <section className="container py-12 sm:py-16">
        <div className="mb-8 flex items-end justify-between">
          <h2 className="font-grotesk text-2xl sm:text-3xl md:text-4xl font-bold">From the blog</h2>
          <Link to="/blog" className="text-xs sm:text-sm text-primary hover:underline">All posts →</Link>
        </div>
        <div className="space-y-4">
          {blogs.length === 0 && <p className="text-muted-foreground">No posts yet.</p>}
          {blogs.map((b) => (
            <Link key={b.id} to={`/blog/${b.slug}`} className="block rounded-lg border border-border bg-card p-6 card-shadow transition-colors hover:border-primary/40">
              <div className="flex flex-col sm:flex-row items-baseline justify-between gap-2 sm:gap-4">
                <h3 className="font-serif text-lg sm:text-2xl">{b.title}</h3>
                <time className="shrink-0 font-mono text-xs text-muted-foreground">{new Date(b.created_at).toLocaleDateString()}</time>
              </div>
              {b.excerpt && <p className="mt-2 text-sm text-muted-foreground">{b.excerpt}</p>}
            </Link>
          ))}
        </div>
      </section>

      {/* Contact strip */}
      <section className="container py-12 sm:py-16">
        <div className="rounded-lg border border-primary/30 bg-gradient-to-br from-card to-secondary p-6 sm:p-10 text-center">
          <h2 className="font-grotesk text-2xl sm:text-3xl font-bold">Let's build together.</h2>
          <p className="mt-2 text-sm sm:text-base text-muted-foreground">Working on something interesting? Drop your email or send a note.</p>
          <div className="mt-6 flex flex-col sm:flex-row gap-3 justify-center">
            <div className="flex gap-2 w-full sm:w-auto">
              <Input
                type="email"
                placeholder="your@email.com"
                value={email}
                onChange={(e) => setEmail(e.target.value)}
                className="flex-1 sm:flex-initial"
              />
              <Button onClick={handleSubscribe} disabled={subscribing} className="glow-shadow">
                <Heart className="mr-2 h-4 w-4" /> Subscribe
              </Button>
            </div>
            <Button asChild variant="outline" size="lg">
              <a href="mailto:profradon@gmail.com"><Mail className="mr-2 h-4 w-4" /> Email me</a>
            </Button>
          </div>
        </div>
      </section>
    </div>
  );
}



//jhgcbvhbjnkb bhec/koijhgmjhujhg,ujhyvfk,usMVChbNVSCHBjkhgvaSHckjhBASGVjhASGXJgbAV