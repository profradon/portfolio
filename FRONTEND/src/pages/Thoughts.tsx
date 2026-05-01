import { useEffect, useState } from "react";
import { api } from "@/integrations/client";
import { RichText } from "@/components/RichText";
import type { Thought } from "@/integrations/types";

export default function Thoughts() {
  const [items, setItems] = useState<Thought[]>([]);
  useEffect(() => {
    document.title = "Thoughts — Prof Radon";
    api.get<Thought[]>("/api/thoughts").then(({ data }) => setItems(data || []));
  }, []);

  return (
    <div className="container py-16">
      <header className="mb-12">
        <p className="font-mono text-xs uppercase tracking-[0.25em] text-primary">// stream</p>
        <h1 className="mt-3 font-grotesk text-5xl font-bold">Random thoughts</h1>
        <p className="mt-3 max-w-2xl text-muted-foreground">Half-formed ideas, observations, things mid-baking.</p>
      </header>
      <div className="space-y-6">
        {items.length === 0 && <p className="text-muted-foreground">No thoughts yet.</p>}
        {items.map((t) => (
          <article key={t.id} className="rounded-lg border border-border bg-card p-6 card-shadow">
            <time className="font-mono text-xs text-muted-foreground">{new Date(t.created_at).toLocaleString()}</time>
            <div className="mt-2 font-lora text-lg">
              <RichText html={t.content} />
            </div>
          </article>
        ))}
      </div>
    </div>
  );
}
