import { useEffect, useState } from "react";
import { api } from "@/integrations/client";
import { RichText } from "@/components/RichText";
import { Github, Mail, Phone } from "lucide-react";
import type { About } from "@/integrations/types";

export default function About() {
  const [content, setContent] = useState<string>("");
  useEffect(() => {
    document.title = "About — Prof Radon";
    api.get<About>("/api/about").then(({ data }) => setContent(data?.content || ""));
  }, []);

  return (
    <div className="container max-w-3xl py-16">
      <p className="font-mono text-xs uppercase tracking-[0.25em] text-primary">// about</p>
      <h1 className="mt-3 font-grotesk text-5xl font-bold">About me</h1>

      <div className="mt-8 font-lora text-lg leading-relaxed">
        <RichText html={content} />
      </div>

      <div className="mt-12 rounded-lg border border-border bg-card p-6 card-shadow">
        <p className="font-mono text-xs uppercase tracking-[0.2em] text-primary">Contact</p>
        <ul className="mt-4 space-y-2 text-sm">
          <li className="flex items-center gap-2"><Mail className="h-4 w-4 text-primary" /> <a href="mailto:profradon@gmail.com" className="hover:underline">profradon@gmail.com</a></li>
          <li className="flex items-center gap-2"><Phone className="h-4 w-4 text-primary" /> <a href="tel:+2349118932656" className="hover:underline">09118932656</a></li>
          <li className="flex items-center gap-2"><Github className="h-4 w-4 text-primary" /> <a href="https://github.com/rustyRadon" target="_blank" rel="noreferrer" className="hover:underline">github.com/rustyRadon</a></li>
        </ul>
      </div>
    </div>
  );
}
