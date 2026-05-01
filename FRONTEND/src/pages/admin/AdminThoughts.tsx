import { useEffect, useState } from "react";
import { api } from "@/integrations/client";
import { RichEditor } from "@/components/RichEditor";
import { RichText } from "@/components/RichText";
import { Button } from "@/components/ui/button";
import { Dialog, DialogContent, DialogHeader, DialogTitle, DialogFooter } from "@/components/ui/dialog";
import { toast } from "sonner";
import { Plus, Trash2, Pencil, Loader2 } from "lucide-react";
import type { Thought } from "@/integrations/types";

export default function AdminThoughts() {
  const [items, setItems] = useState<Thought[]>([]);
  const [loading, setLoading] = useState(true);
  const [open, setOpen] = useState(false);
  const [editing, setEditing] = useState<Thought | null>(null);
  const [content, setContent] = useState("");
  const [saving, setSaving] = useState(false);

  const load = async () => {
    setLoading(true);
    const { data } = await api.get<Thought[]>("/api/admin/thoughts");
    setItems(data || []); setLoading(false);
  };
  useEffect(() => { load(); }, []);

  const startNew = () => { setEditing(null); setContent(""); setOpen(true); };
  const startEdit = (t: Thought) => { setEditing(t); setContent(t.content); setOpen(true); };

  const save = async () => {
    if (!content.trim() || content === "<p></p>") { toast.error("Write something"); return; }
    setSaving(true);
    const { error } = editing
      ? await api.put(`/api/admin/thoughts/${editing.id}`, { content })
      : await api.post("/api/admin/thoughts", { content });
    setSaving(false);
    if (error) return toast.error(error);
    toast.success("Saved"); setOpen(false); load();
  };

  const remove = async (id: string) => {
    if (!confirm("Delete?")) return;
    const { error } = await api.delete(`/api/admin/thoughts/${id}`);
    if (error) toast.error(error); else { toast.success("Deleted"); load(); }
  };

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <div><h1 className="font-grotesk text-3xl font-bold">Thoughts</h1><p className="mt-1 text-sm text-muted-foreground">Quick rich-text musings.</p></div>
        <Button onClick={startNew} className="glow-shadow"><Plus className="mr-1 h-4 w-4" /> New</Button>
      </div>

      {loading ? <Loader2 className="h-6 w-6 animate-spin" /> : (
        <div className="space-y-3">
          {items.length === 0 && <p className="text-sm text-muted-foreground">No thoughts yet.</p>}
          {items.map((t) => (
            <div key={t.id} className="rounded-lg border border-border bg-card p-4 card-shadow">
              <div className="flex items-start justify-between gap-4">
                <time className="font-mono text-xs text-muted-foreground">{new Date(t.created_at).toLocaleString()}</time>
                <div className="flex shrink-0 gap-1">
                  <Button size="icon" variant="ghost" onClick={() => startEdit(t)}><Pencil className="h-4 w-4" /></Button>
                  <Button size="icon" variant="ghost" onClick={() => remove(t.id)}><Trash2 className="h-4 w-4 text-destructive" /></Button>
                </div>
              </div>
              <div className="mt-2"><RichText html={t.content} /></div>
            </div>
          ))}
        </div>
      )}

      <Dialog open={open} onOpenChange={setOpen}>
        <DialogContent className="max-w-3xl">
          <DialogHeader><DialogTitle>{editing ? "Edit thought" : "New thought"}</DialogTitle></DialogHeader>
          <RichEditor value={content} onChange={setContent} placeholder="What's on your mind?" minHeight={200} />
          <DialogFooter>
            <Button variant="ghost" onClick={() => setOpen(false)}>Cancel</Button>
            <Button onClick={save} disabled={saving}>{saving ? <Loader2 className="h-4 w-4 animate-spin" /> : "Save"}</Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
    </div>
  );
}
