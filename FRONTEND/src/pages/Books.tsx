import { useEffect, useState } from "react";
import { api } from "@/integrations/client";
import { ExternalLink } from "lucide-react";
import type { Book } from "@/integrations/types";

export default function Books() {
  const [items, setItems] = useState<Book[]>([]);
  useEffect(() => {
    document.title = "Books — Prof Radon";
    api.get<Book[]>("/api/books").then(({ data }) => setItems(data || []));
  }, []);

  return (
    <div className="container py-16">
      <header className="mb-12">
        <p className="font-mono text-xs uppercase tracking-[0.25em] text-primary">// reading list</p>
        <h1 className="mt-3 font-grotesk text-5xl font-bold">Books I recommend</h1>
        <p className="mt-3 max-w-2xl text-muted-foreground">A growing pile of titles that shaped how I think.</p>
      </header>
      <div className="grid gap-6 sm:grid-cols-2 lg:grid-cols-3">
        {items.length === 0 && <p className="text-muted-foreground">No books yet.</p>}
        {items.map((b) => (
          <article key={b.id} className="rounded-lg border border-border bg-card p-5 card-shadow">
            <div className="flex gap-4">
              {b.cover_url ? (
                <img src={b.cover_url} alt={b.title} className="h-28 w-20 shrink-0 rounded object-cover" loading="lazy" />
              ) : (
                <div className="flex h-28 w-20 shrink-0 items-center justify-center rounded bg-secondary font-serif text-base text-muted-foreground">No cover</div>
              )}
              <div className="min-w-0">
                <h2 className="font-serif text-lg leading-tight">{b.title}</h2>
                <p className="mt-1 text-xs text-muted-foreground">{b.author}</p>
                {b.link && (
                  <a href={b.link} target="_blank" rel="noreferrer" className="mt-2 inline-flex items-center gap-1 text-xs text-primary hover:underline">
                    Find it <ExternalLink className="h-3 w-3" />
                  </a>
                )}
              </div>
            </div>
            {b.notes && <p className="mt-3 text-sm text-muted-foreground">{b.notes}</p>}
          </article>
        ))}
      </div>
    </div>
  );
}
