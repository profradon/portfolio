import { useEffect, useState } from "react";
import { api } from "@/integrations/client";
import { ArrowUpRight } from "lucide-react";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select";
import type { Project } from "@/integrations/types";

const PROJECT_TYPES = [
  "Web Development",
  "App Development",
  "System Development",
  "Networking",
  "Upwork",
  "Open Source",
  "Learning",
];

const LANGUAGES = [
  "JavaScript",
  "TypeScript",
  "Rust",
  "C",
  "C++",
  "Python",
  "Go",
  "Solidity",
];

const FRAMEWORKS = [
  "React",
  "Express.js",
  "Next.js",
  "Slint",
  "Axum",
  "FastAPI",
  "Vue.js",
];

export default function Projects() {
  const [items, setItems] = useState<Project[]>([]);
  const [filtered, setFiltered] = useState<Project[]>([]);
  const [selectedTypes, setSelectedTypes] = useState<string[]>([]);
  const [selectedLanguages, setSelectedLanguages] = useState<string[]>([]);
  const [selectedFrameworks, setSelectedFrameworks] = useState<string[]>([]);
  const [search, setSearch] = useState("");
  const [expandedProjects, setExpandedProjects] = useState<Record<string, boolean>>({});

  const toggleProject = (id: string) => {
    setExpandedProjects((prev) => ({ ...prev, [id]: !prev[id] }));
  };

  useEffect(() => {
    document.title = "Projects — Prof Radon";
    api.get<Project[]>("/api/projects").then(({ data }) => {
      setItems(data || []);
      setFiltered(data || []);
    });
  }, []);

  useEffect(() => {
    let result = items;

    if (search) {
      result = result.filter(
        (p) =>
          p.title.toLowerCase().includes(search.toLowerCase()) ||
          p.description.toLowerCase().includes(search.toLowerCase())
      );
    }

    if (selectedTypes.length > 0) {
      result = result.filter((p) =>
        p.project_types?.some((t) => selectedTypes.includes(t))
      );
    }

    if (selectedLanguages.length > 0) {
      result = result.filter((p) =>
        p.languages?.some((l) => selectedLanguages.includes(l))
      );
    }

    if (selectedFrameworks.length > 0) {
      result = result.filter((p) =>
        selectedFrameworks.some((f) => p.technologies?.includes(f))
      );
    }

    setFiltered(result);
  }, [search, selectedTypes, selectedLanguages, selectedFrameworks, items]);

  const toggleType = (type: string) => {
    setSelectedTypes(
      selectedTypes.includes(type)
        ? selectedTypes.filter((t) => t !== type)
        : [...selectedTypes, type]
    );
  };

  const toggleLanguage = (lang: string) => {
    setSelectedLanguages(
      selectedLanguages.includes(lang)
        ? selectedLanguages.filter((l) => l !== lang)
        : [...selectedLanguages, lang]
    );
  };

  const toggleFramework = (fw: string) => {
    setSelectedFrameworks(
      selectedFrameworks.includes(fw)
        ? selectedFrameworks.filter((f) => f !== fw)
        : [...selectedFrameworks, fw]
    );
  };

  const clearFilters = () => {
    setSelectedTypes([]);
    setSelectedLanguages([]);
    setSelectedFrameworks([]);
    setSearch("");
  };

  return (
    <div className="container py-8 sm:py-16">
      <header className="mb-8 sm:mb-12">
        <p className="font-mono text-xs uppercase tracking-[0.25em] text-primary">// projects</p>
        <h1 className="mt-3 font-grotesk text-3xl sm:text-4xl md:text-5xl font-bold">Things I've built</h1>
      </header>

      {/* Filters */}
      <div className="mb-8 space-y-4">
        <Input
          placeholder="Search projects..."
          value={search}
          onChange={(e) => setSearch(e.target.value)}
          className="w-full"
        />

        <div className="space-y-3">
          <div>
            <label className="block text-sm font-medium mb-2">Project Type</label>
            <div className="flex flex-wrap gap-2">
              {PROJECT_TYPES.map((type) => (
                <Button
                  key={type}
                  variant={selectedTypes.includes(type) ? "default" : "outline"}
                  size="sm"
                  onClick={() => toggleType(type)}
                  className="text-xs"
                >
                  {type}
                </Button>
              ))}
            </div>
          </div>

          <div>
            <label className="block text-sm font-medium mb-2">Language</label>
            <div className="flex flex-wrap gap-2">
              {LANGUAGES.map((lang) => (
                <Button
                  key={lang}
                  variant={selectedLanguages.includes(lang) ? "default" : "outline"}
                  size="sm"
                  onClick={() => toggleLanguage(lang)}
                  className="text-xs"
                >
                  {lang}
                </Button>
              ))}
            </div>
          </div>

          <div>
            <label className="block text-sm font-medium mb-2">Framework</label>
            <div className="flex flex-wrap gap-2">
              {FRAMEWORKS.map((fw) => (
                <Button
                  key={fw}
                  variant={selectedFrameworks.includes(fw) ? "default" : "outline"}
                  size="sm"
                  onClick={() => toggleFramework(fw)}
                  className="text-xs"
                >
                  {fw}
                </Button>
              ))}
            </div>
          </div>
        </div>

        {(search || selectedTypes.length > 0 || selectedLanguages.length > 0 || selectedFrameworks.length > 0) && (
          <Button variant="ghost" size="sm" onClick={clearFilters}>Clear all</Button>
        )}
      </div>

      {/* Results */}
      <div className="grid gap-6 sm:grid-cols-2">
        {filtered.length === 0 && <p className="text-muted-foreground">No projects found.</p>}
        {filtered.map((p) => (
          <article
            key={p.id}
            className="group rounded-lg border border-border bg-card p-6 card-shadow transition-all hover:-translate-y-1 hover:border-primary/40"
          >
            {p.image_url && (
              <img
                src={p.image_url}
                alt={p.title}
                className="mb-4 aspect-video w-full rounded-md object-cover"
                loading="lazy"
              />
            )}
            <div className="flex items-start justify-between gap-3">
              <h2 className="font-grotesk text-xl sm:text-2xl font-semibold">{p.title}</h2>
              {p.live_url && (
                <a
                  href={p.live_url}
                  target="_blank"
                  rel="noreferrer"
                  className="text-muted-foreground transition-colors hover:text-primary"
                >
                  <ArrowUpRight className="h-5 w-5" />
                </a>
              )}
            </div>
            {(() => {
              const projectDescription = p.long_description || p.description;
              const isExpanded = expandedProjects[p.id];
              const shouldTruncate = projectDescription.length > 180;

              return (
                <>
                  <p className="mt-2 text-sm text-muted-foreground">
                    {shouldTruncate && !isExpanded
                      ? `${projectDescription.slice(0, 180).trim()}...`
                      : projectDescription}
                  </p>
                  {shouldTruncate && (
                    <button
                      type="button"
                      onClick={() => toggleProject(p.id)}
                      className="mt-2 text-sm text-primary hover:underline"
                    >
                      {isExpanded ? "Show less" : "Read more"}
                    </button>
                  )}
                </>
              );
            })()}

            {(p.project_types?.length > 0 || p.languages?.length > 0) && (
              <div className="mt-3 space-y-2 text-xs">
                {p.project_types?.length > 0 && (
                  <div>
                    <span className="font-semibold text-muted-foreground">Type:</span> {p.project_types.join(", ")}
                  </div>
                )}
                {p.languages?.length > 0 && (
                  <div>
                    <span className="font-semibold text-muted-foreground">Languages:</span> {p.languages.join(", ")}
                  </div>
                )}
              </div>
            )}

            <div className="mt-4 flex flex-wrap gap-1.5">
              {p.technologies?.slice(0, 6).map((t) => (
                <span
                  key={t}
                  className="rounded-full bg-secondary px-2 py-0.5 font-mono text-xs text-muted-foreground"
                >
                  {t}
                </span>
              ))}
            </div>
          </article>
        ))}
      </div>
    </div>
  );
}
