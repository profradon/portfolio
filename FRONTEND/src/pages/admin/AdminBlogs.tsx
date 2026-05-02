import { useEffect, useState } from "react";
import { api } from "@/integrations/client";
import { RichEditor } from "@/components/RichEditor";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Textarea } from "@/components/ui/textarea";
import { Switch } from "@/components/ui/switch";
import { Dialog, DialogContent, DialogHeader, DialogTitle, DialogDescription, DialogFooter } from "@/components/ui/dialog";
import { toast } from "sonner";
import { Plus, Trash2, Pencil, Loader2, Eye } from "lucide-react";
import { slugify } from "@/lib/slug";
import { Link } from "react-router-dom";
import type { Blog } from "@/integrations/types";

const empty = { slug: "", title: "", excerpt: "", content: "", cover_url: "", published: true };

export default function AdminBlogs() {
  const [items, setItems] = useState<Blog[]>([]);
  const [loading, setLoading] = useState(true);
  const [open, setOpen] = useState(false);
  const [editing, setEditing] = useState<Blog | null>(null);
  const [form, setForm] = useState(empty);
  const [slugTouched, setSlugTouched] = useState(false);
  const [saving, setSaving] = useState(false);
  const [coverPreview, setCoverPreview] = useState("");

  const load = async () => {
    setLoading(true);
    const { data } = await api.get<Blog[]>("/api/admin/blogs");
    setItems(data || []); setLoading(false);
  };
  useEffect(() => { load(); }, []);

  const startNew = () => { setEditing(null); setForm(empty); setSlugTouched(false); setCoverPreview(""); setOpen(true); };
  const startEdit = (b: Blog) => {
    setEditing(b); setSlugTouched(true);
    setForm({ slug: b.slug, title: b.title, excerpt: b.excerpt, content: b.content, cover_url: b.cover_url || "", published: b.published });
    setCoverPreview(b.cover_url || "");
    setOpen(true);
  };

  const onTitle = (title: string) => {
    setForm((f) => ({ ...f, title, slug: slugTouched ? f.slug : slugify(title) }));
  };

  const save = async () => {
    if (!form.title.trim()) return toast.error("Title required");
    if (!form.slug.trim()) return toast.error("Slug required");
    setSaving(true);
    const payload = { ...form, slug: slugify(form.slug), cover_url: form.cover_url || null };
    const { error } = editing
      ? await api.put(`/api/admin/blogs/${editing.id}`, payload)
      : await api.post("/api/admin/blogs", payload);
    setSaving(false);
    if (error) return toast.error(error);
    toast.success("Saved"); setOpen(false); load();
  };

  const remove = async (id: string) => {
    if (!confirm("Delete this post?")) return;
    const { error } = await api.delete(`/api/admin/blogs/${id}`);
    if (error) toast.error(error); else { toast.success("Deleted"); load(); }
  };

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <div><h1 className="font-grotesk text-3xl font-bold">Blog posts</h1><p className="mt-1 text-sm text-muted-foreground">Write rich-text posts with custom fonts.</p></div>
        <Button onClick={startNew} className="glow-shadow"><Plus className="mr-1 h-4 w-4" /> New post</Button>
      </div>

      {loading ? <Loader2 className="h-6 w-6 animate-spin" /> : (
        <div className="space-y-3">
          {items.length === 0 && <p className="text-sm text-muted-foreground">No posts yet.</p>}
          {items.map((b) => (
            <div key={b.id} className="flex items-start justify-between gap-4 rounded-lg border border-border bg-card p-4 card-shadow">
              <div className="min-w-0">
                <div className="flex items-center gap-2">
                  <h3 className="font-serif text-lg">{b.title}</h3>
                  {!b.published && <span className="rounded bg-secondary px-1.5 py-0.5 font-mono text-[10px] uppercase text-muted-foreground">draft</span>}
                </div>
                <p className="font-mono text-xs text-muted-foreground">/{b.slug}</p>
              </div>
              <div className="flex shrink-0 gap-1">
                {b.published && <Button size="icon" variant="ghost" asChild><Link to={`/blog/${b.slug}`} target="_blank"><Eye className="h-4 w-4" /></Link></Button>}
                <Button size="icon" variant="ghost" onClick={() => startEdit(b)}><Pencil className="h-4 w-4" /></Button>
                <Button size="icon" variant="ghost" onClick={() => remove(b.id)}><Trash2 className="h-4 w-4 text-destructive" /></Button>
              </div>
            </div>
          ))}
        </div>
      )}

      <Dialog open={open} onOpenChange={setOpen}>
        <DialogContent className="max-h-[90vh] max-w-4xl overflow-y-auto">
          <DialogHeader>
            <DialogTitle>{editing ? "Edit post" : "New post"}</DialogTitle>
            <DialogDescription>Choose a cover image or upload one from your computer, then save the post.</DialogDescription>
          </DialogHeader>
          <div className="space-y-4">
            <div><Label>Title</Label><Input value={form.title} onChange={(e) => onTitle(e.target.value)} /></div>
            <div className="grid grid-cols-2 gap-3">
              <div><Label>Slug</Label><Input value={form.slug} onChange={(e) => { setSlugTouched(true); setForm({ ...form, slug: e.target.value }); }} /></div>
              <div>
                <Label>Cover image URL</Label>
                <Input value={form.cover_url} onChange={(e) => { setForm({ ...form, cover_url: e.target.value }); setCoverPreview(e.target.value); }} />
              </div>
            </div>
            <div>
              <Label>Or upload cover image</Label>
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
                      setForm({ ...form, cover_url: reader.result });
                      setCoverPreview(reader.result);
                    }
                  };
                  reader.readAsDataURL(file);
                }}
              />
              {coverPreview && <img src={coverPreview} alt="Cover preview" className="mt-3 h-40 w-full rounded-md object-cover" />}
            </div>
            <div><Label>Excerpt</Label><Textarea rows={2} value={form.excerpt} onChange={(e) => setForm({ ...form, excerpt: e.target.value })} /></div>
            <div>
              <Label>Content</Label>
              <div className="mt-1.5">
                <RichEditor value={form.content} onChange={(html) => setForm({ ...form, content: html })} placeholder="Write your post…" minHeight={350} />
              </div>
            </div>
            <div className="flex items-center gap-3">
              <Switch checked={form.published} onCheckedChange={(v) => setForm({ ...form, published: v })} />
              <Label>Published</Label>
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
