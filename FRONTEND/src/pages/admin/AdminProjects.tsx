import { useEffect, useState } from "react";
import { api } from "@/integrations/client";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Textarea } from "@/components/ui/textarea";
import { Dialog, DialogContent, DialogHeader, DialogTitle, DialogDescription, DialogFooter } from "@/components/ui/dialog";
import { toast } from "sonner";
import { Plus, Trash2, Pencil, Loader2 } from "lucide-react";
import type { Project } from "@/integrations/types";

const PROJECT_TYPES = ["Web Development", "App Development", "System Development", "Networking", "Upwork", "Open Source", "Learning"];
const LANGUAGES = ["JavaScript", "TypeScript", "Rust", "C", "C++", "Python", "Go", "Solidity"];
const FRAMEWORKS = ["React", "Express.js", "Next.js", "Slint", "Axum", "FastAPI", "Vue.js"];

const empty = { title: "", description: "", long_description: "", live_url: "", github_url: "", image_url: "", technologies: "", project_types: "", languages: "", featured: false };

export default function AdminProjects() {
  const [items, setItems] = useState<Project[]>([]);
  const [loading, setLoading] = useState(true);
  const [open, setOpen] = useState(false);
  const [editing, setEditing] = useState<Project | null>(null);
  const [form, setForm] = useState(empty);
  const [saving, setSaving] = useState(false);
  const [imagePreview, setImagePreview] = useState("");

  const load = async () => {
    setLoading(true);
    const { data } = await api.get<Project[]>('/api/admin/projects');
    setItems(data || []);
    setLoading(false);
  };

  useEffect(() => { load(); }, []);

  const startNew = () => { setEditing(null); setForm(empty); setImagePreview(""); setOpen(true); };
  const startEdit = (p: Project) => {
    setEditing(p);
    setForm({
      title: p.title,
      description: p.description,
      long_description: p.long_description || "",
      live_url: p.live_url || "",
      github_url: p.github_url || "",
      image_url: p.image_url || "",
      technologies: p.technologies?.join(", ") || "",
      project_types: p.project_types?.join(", ") || "",
      languages: p.languages?.join(", ") || "",
      featured: p.featured,
    });
    setImagePreview(p.image_url || "");
    setOpen(true);
  };

  const save = async () => {
    setSaving(true);
    try {
      const payload = {
        title: form.title.trim(),
        description: form.description,
        long_description: form.long_description || null,
        live_url: form.live_url || null,
        github_url: form.github_url || null,
        image_url: form.image_url || null,
        technologies: form.technologies.split(",").map((t) => t.trim()).filter(Boolean),
        project_types: form.project_types.split(",").map((t) => t.trim()).filter(Boolean),
        languages: form.languages.split(",").map((l) => l.trim()).filter(Boolean),
        featured: form.featured,
      };
      if (!payload.title) { toast.error("Title required"); setSaving(false); return; }
      const { error } = editing
        ? await api.put(`/api/admin/projects/${editing.id}`, payload)
        : await api.post("/api/admin/projects", payload);
      if (error) throw new Error(error);
      toast.success(editing ? "Updated" : "Created");
      setOpen(false);
      load();
    } catch (e) { toast.error(e instanceof Error ? e.message : "Failed"); }
    finally { setSaving(false); }
  };

  const remove = async (id: string) => {
    if (!confirm("Delete this project?")) return;
    const { error } = await api.delete(`/api/admin/projects/${id}`);
    if (error) toast.error(error); else { toast.success("Deleted"); load(); }
  };

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <div>
          <h1 className="font-grotesk text-3xl font-bold">Projects</h1>
          <p className="mt-1 text-sm text-muted-foreground">Manage portfolio projects.</p>
        </div>
        <Button onClick={startNew} className="glow-shadow"><Plus className="mr-1 h-4 w-4" /> New</Button>
      </div>

      {loading ? <Loader2 className="h-6 w-6 animate-spin" /> : (
        <div className="space-y-3">
          {items.length === 0 && <p className="text-sm text-muted-foreground">No projects yet.</p>}
          {items.map((p) => (
            <div key={p.id} className="flex items-start justify-between gap-4 rounded-lg border border-border bg-card p-4 card-shadow">
              <div className="min-w-0">
                <h3 className="font-grotesk font-semibold">{p.title}</h3>
                <p className="mt-1 line-clamp-2 text-sm text-muted-foreground">{p.description}</p>
              </div>
              <div className="flex shrink-0 gap-1">
                <Button size="icon" variant="ghost" onClick={() => startEdit(p)}><Pencil className="h-4 w-4" /></Button>
                <Button size="icon" variant="ghost" onClick={() => remove(p.id)}><Trash2 className="h-4 w-4 text-destructive" /></Button>
              </div>
            </div>
          ))}
        </div>
      )}

      <Dialog open={open} onOpenChange={setOpen}>
        <DialogContent className="max-w-2xl max-h-[90vh] overflow-y-auto">
          <DialogHeader>
            <DialogTitle>{editing ? "Edit project" : "New project"}</DialogTitle>
            <DialogDescription>
              Add a project with its type, languages, and an optional image. Select from the available filters to keep results consistent.
            </DialogDescription>
          </DialogHeader>
          <div className="space-y-4">
            <div><Label>Title</Label><Input value={form.title} onChange={(e) => setForm({ ...form, title: e.target.value })} /></div>
            <div><Label>Description</Label><Textarea rows={3} value={form.description} onChange={(e) => setForm({ ...form, description: e.target.value })} /></div>
            <div><Label>Long Description</Label><Textarea rows={3} value={form.long_description} onChange={(e) => setForm({ ...form, long_description: e.target.value })} /></div>
            <div className="grid grid-cols-2 gap-3">
              <div><Label>GitHub URL</Label><Input placeholder="https://…" value={form.github_url} onChange={(e) => setForm({ ...form, github_url: e.target.value })} /></div>
              <div><Label>Live URL</Label><Input placeholder="https://…" value={form.live_url} onChange={(e) => setForm({ ...form, live_url: e.target.value })} /></div>
            </div>
            <div>
              <Label>Image</Label>
              <div className="flex flex-col gap-2">
                <Input placeholder="https://…" value={form.image_url} onChange={(e) => { setForm({ ...form, image_url: e.target.value }); setImagePreview(e.target.value); }} />
                <input
                  type="file"
                  accept="image/*"
                  className="rounded border border-border bg-background px-3 py-2"
                  onChange={(e) => {
                    const file = e.target.files?.[0];
                    if (!file) return;
                    const reader = new FileReader();
                    reader.onload = () => {
                      if (typeof reader.result === "string") {
                        setImagePreview(reader.result);
                        setForm({ ...form, image_url: reader.result });
                      }
                    };
                    reader.readAsDataURL(file);
                  }}
                />
                {imagePreview && (
                  <img src={imagePreview} alt="Project preview" className="h-40 w-full rounded-md object-cover" />
                )}
              </div>
            </div>
            <div><Label>Technologies</Label><Input placeholder="React, Express, TypeScript..." value={form.technologies} onChange={(e) => setForm({ ...form, technologies: e.target.value })} /></div>
            <div>
              <Label>Project Types</Label>
              <select
                multiple
                value={form.project_types.split(",").map((value) => value.trim()).filter(Boolean)}
                onChange={(e) => setForm({ ...form, project_types: Array.from(e.target.selectedOptions).map((option) => option.value).join(", ") })}
                className="w-full rounded border border-border bg-background px-3 py-2"
                size={Math.min(PROJECT_TYPES.length, 6)}
              >
                {PROJECT_TYPES.map((type) => (
                  <option key={type} value={type}>{type}</option>
                ))}
              </select>
              <p className="mt-1 text-xs text-muted-foreground">Hold Ctrl/Cmd to select multiple types.</p>
            </div>
            <div>
              <Label>Languages</Label>
              <select
                multiple
                value={form.languages.split(",").map((value) => value.trim()).filter(Boolean)}
                onChange={(e) => setForm({ ...form, languages: Array.from(e.target.selectedOptions).map((option) => option.value).join(", ") })}
                className="w-full rounded border border-border bg-background px-3 py-2"
                size={Math.min(LANGUAGES.length, 6)}
              >
                {LANGUAGES.map((lang) => (
                  <option key={lang} value={lang}>{lang}</option>
                ))}
              </select>
              <p className="mt-1 text-xs text-muted-foreground">Hold Ctrl/Cmd to select multiple languages.</p>
            </div>
            <div className="flex items-center gap-2">
              <input type="checkbox" id="featured" checked={form.featured} onChange={(e) => setForm({ ...form, featured: e.target.checked })} />
              <Label htmlFor="featured">Featured</Label>
            </div>
          </div>
          <DialogFooter>
            <Button variant="ghost" onClick={() => setOpen(false)}>Cancel</Button>
            <Button onClick={save} disabled={saving}>{saving ? <Loader2 className="h-4 w-4 animate-spin" /> : "Save"}</Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
    </div>
  );
}
