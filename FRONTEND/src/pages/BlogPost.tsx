import { useEffect, useState } from "react";
import { Link, useParams } from "react-router-dom";
import { api } from "@/integrations/client";
import { RichText } from "@/components/RichText";
import { ArrowLeft } from "lucide-react";
import type { Blog } from "@/integrations/types";

export default function BlogPost() {
  const { slug } = useParams();
  const [post, setPost] = useState<Blog | null>(null);
  const [notFound, setNotFound] = useState(false);

  useEffect(() => {
    if (!slug) return;
    api.get<Blog>(`/api/blogs/${slug}`)
      .then(({ data, error }) => {
        if (error || !data) {
          setNotFound(true);
          return;
        }
        setPost(data);
        document.title = `${data.title} — Prof Radon`;
        const meta = document.querySelector('meta[name="description"]') || document.head.appendChild(Object.assign(document.createElement("meta"), { name: "description" }));
        meta.setAttribute("content", data.excerpt || data.title);
      });
  }, [slug]);

  if (notFound) return (
    <div className="container max-w-3xl py-24 text-center">
      <h1 className="font-grotesk text-4xl">Not found</h1>
      <Link to="/blog" className="mt-4 inline-block text-primary hover:underline">← Back to blog</Link>
    </div>
  );
  if (!post) return <div className="container py-24" />;

  return (
    <article className="container max-w-3xl py-16">
      <Link to="/blog" className="inline-flex items-center gap-1 text-sm text-muted-foreground transition-colors hover:text-primary">
        <ArrowLeft className="h-4 w-4" /> Back to blog
      </Link>
      <header className="mt-8">
        <time className="font-mono text-xs text-muted-foreground">{new Date(post.created_at).toLocaleDateString()}</time>
        <h1 className="mt-3 font-serif text-5xl leading-tight">{post.title}</h1>
        {post.excerpt && <p className="mt-4 font-lora text-xl text-muted-foreground">{post.excerpt}</p>}
      </header>
      {post.cover_url && <img src={post.cover_url} alt={post.title} className="mt-8 w-full rounded-lg" />}
      <div className="mt-10 font-lora text-lg leading-relaxed">
        <RichText html={post.content} />
      </div>
    </article>
  );
}

///jlhkgfcdxgsgxfchjbknbhv.ihgyuihyugnfchyuligkjchmvhsukygjckh