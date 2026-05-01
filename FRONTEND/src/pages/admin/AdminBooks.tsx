import { useEffect, useState } from "react";
import { api } from "@/integrations/client";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Textarea } from "@/components/ui/textarea";
import { Dialog, DialogContent, DialogHeader, DialogTitle, DialogFooter } from "@/components/ui/dialog";
import { toast } from "sonner";
import { Plus, Trash2, Pencil, Loader2 } from "lucide-react";
import type { Book } from "@/integrations/types";

const empty = { title: "", author: "", notes: "", cover_url: "", link: "" };

export default function AdminBooks() {
  const [items, setItems] = useState<Book[]>([]);
  const [loading, setLoading] = useState(true);
  const [open, setOpen] = useState(false);
  const [editing, setEditing] = useState<Book | null>(null);
  const [form, setForm] = useState(empty);
  const [saving, setSaving] = useState(false);

  const load = async () => {
    setLoading(true);
    const { data } = await api.get<Book[]>("/api/admin/books");
    setItems(data || []); setLoading(false);
  };
  useEffect(() => { load(); }, []);

  const startNew = () => { setEditing(null); setForm(empty); setOpen(true); };
  const startEdit = (b: Book) => {
    setEditing(b);
    setForm({ title: b.title, author: b.author, notes: b.notes, cover_url: b.cover_url || "", link: b.link || "" });
    setOpen(true);
  };

  const save = async () => {
    if (!form.title.trim()) { toast.error("Title required"); return; }
    setSaving(true);
    const payload = { ...form, cover_url: form.cover_url || null, link: form.link || null };
    const { error } = editing
      ? await api.put(`/api/admin/books/${editing.id}`, payload)
      : await api.post("/api/admin/books", payload);
    setSaving(false);
    if (error) return toast.error(error);
    toast.success("Saved"); setOpen(false); load();
  };

  const remove = async (id: string) => {
    if (!confirm("Delete?")) return;
    const { error } = await api.delete(`/api/admin/books/${id}`);
    if (error) toast.error(error); else { toast.success("Deleted"); load(); }
  };

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <div><h1 className="font-grotesk text-3xl font-bold">Books</h1><p className="mt-1 text-sm text-muted-foreground">Suggested reading.</p></div>
        <Button onClick={startNew} className="glow-shadow"><Plus className="mr-1 h-4 w-4" /> New</Button>
      </div>

      {loading ? <Loader2 className="h-6 w-6 animate-spin" /> : (
        <div className="space-y-3">
          {items.length === 0 && <p className="text-sm text-muted-foreground">No books yet.</p>}
          {items.map((b) => (
            <div key={b.id} className="flex items-start justify-between gap-4 rounded-lg border border-border bg-card p-4 card-shadow">
              <div className="min-w-0">
                <h3 className="font-serif text-lg">{b.title}</h3>
                <p className="text-xs text-muted-foreground">{b.author}</p>
              </div>
              <div className="flex shrink-0 gap-1">
                <Button size="icon" variant="ghost" onClick={() => startEdit(b)}><Pencil className="h-4 w-4" /></Button>
                <Button size="icon" variant="ghost" onClick={() => remove(b.id)}><Trash2 className="h-4 w-4 text-destructive" /></Button>
              </div>
            </div>
          ))}
        </div>
      )}

      <Dialog open={open} onOpenChange={setOpen}>
        <DialogContent>
          <DialogHeader><DialogTitle>{editing ? "Edit book" : "New book"}</DialogTitle></DialogHeader>
          <div className="space-y-4">
            <div><Label>Title</Label><Input value={form.title} onChange={(e) => setForm({ ...form, title: e.target.value })} /></div>
            <div><Label>Author</Label><Input value={form.author} onChange={(e) => setForm({ ...form, author: e.target.value })} /></div>
            <div><Label>Cover URL</Label><Input value={form.cover_url} onChange={(e) => setForm({ ...form, cover_url: e.target.value })} /></div>
            <div><Label>Link</Label><Input value={form.link} onChange={(e) => setForm({ ...form, link: e.target.value })} /></div>
            <div><Label>Notes</Label><Textarea rows={3} value={form.notes} onChange={(e) => setForm({ ...form, notes: e.target.value })} /></div>
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
