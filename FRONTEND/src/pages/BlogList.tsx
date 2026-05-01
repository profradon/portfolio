import { useEffect, useState } from "react";
import { Link } from "react-router-dom";
import { api } from "@/integrations/client";
import type { Blog } from "@/integrations/types";

export default function BlogList() {
  const [items, setItems] = useState<Blog[]>([]);
  useEffect(() => {
    document.title = "Blog — Prof Radon";
    api.get<Blog[]>("/api/blogs")
      .then(({ data }) => setItems(data || []));
  }, []);

  return (
    <div className="container max-w-3xl py-16">
      <header className="mb-12">
        <p className="font-mono text-xs uppercase tracking-[0.25em] text-primary">// writing</p>
        <h1 className="mt-3 font-grotesk text-5xl font-bold">Blog</h1>
      </header>
      <div className="space-y-2">
        {items.length === 0 && <p className="text-muted-foreground">No posts yet.</p>}
        {items.map((b) => (
          <Link key={b.id} to={`/blog/${b.slug}`} className="group block rounded-lg border border-transparent p-6 transition-all hover:border-border hover:bg-card">
            <time className="font-mono text-xs text-muted-foreground">{new Date(b.created_at).toLocaleDateString()}</time>
            <h2 className="mt-1 font-serif text-3xl transition-colors group-hover:text-primary">{b.title}</h2>
            {b.excerpt && <p className="mt-2 text-muted-foreground">{b.excerpt}</p>}
          </Link>
        ))}
      </div>
    </div>
  );
}
