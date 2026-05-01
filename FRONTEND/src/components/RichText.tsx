interface Props { html: string; className?: string; }
export function RichText({ html, className = "" }: Props) {
  return <div className={`tiptap-content ${className}`} dangerouslySetInnerHTML={{ __html: html || "" }} />;
}
