import { useEffect, useState } from "react";
import { api } from "@/integrations/client";
import { RichEditor } from "@/components/RichEditor";
import { Button } from "@/components/ui/button";
import { toast } from "sonner";
import { Loader2, Save } from "lucide-react";
import type { About } from "@/integrations/types";

export default function AdminAbout() {
  const [content, setContent] = useState("");
  const [id, setId] = useState<string | null>(null);
  const [saving, setSaving] = useState(false);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    api.get<About>("/api/admin/about").then(({ data }) => {
      if (data) { setId(data.id); setContent(data.content); }
      setLoading(false);
    });
  }, []);

  const save = async () => {
    setSaving(true);
    try {
      const { error } = await api.put("/api/admin/about", { content });
      if (error) throw new Error(error);
      toast.success("About saved");
    } catch (e) {
      toast.error(e instanceof Error ? e.message : "Failed");
    } finally { setSaving(false); }
  };

  if (loading) return <Loader2 className="h-6 w-6 animate-spin" />;

  return (
    <div className="space-y-6">
      <div>
        <h1 className="font-grotesk text-3xl font-bold">About</h1>
        <p className="mt-1 text-sm text-muted-foreground">This appears on your /about page.</p>
      </div>
      <RichEditor value={content} onChange={setContent} placeholder="Write about yourself…" minHeight={400} />
      <Button onClick={save} disabled={saving} className="glow-shadow">
        {saving ? <Loader2 className="mr-2 h-4 w-4 animate-spin" /> : <Save className="mr-2 h-4 w-4" />} Save
      </Button>
    </div>
  );
}
