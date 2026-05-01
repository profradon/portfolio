import { Github, Mail, Phone, MessageCircle } from "lucide-react";
import { useLocation } from "react-router-dom";

export function SiteFooter() {
  const loc = useLocation();
  if (loc.pathname.startsWith("/admin")) return null;

  return (
    <footer className="mt-16 sm:mt-24 border-t border-border/60">
      <div className="container flex flex-col items-start justify-between gap-6 py-8 sm:py-10 sm:flex-row sm:items-center">
        <div>
          <p className="font-grotesk text-sm font-semibold">Prof Radon</p>
          <p className="mt-1 text-xs text-muted-foreground">Built quietly. Shipped loudly.</p>
        </div>
        <div className="flex flex-wrap items-center gap-3 sm:gap-4 text-xs sm:text-sm text-muted-foreground">
          <a className="flex items-center gap-1.5 transition-colors hover:text-primary" href="https://github.com/rustyRadon" target="_blank" rel="noreferrer">
            <Github className="h-4 w-4" /> rustyRadon
          </a>
          <a className="flex items-center gap-1.5 transition-colors hover:text-primary" href="mailto:profradon@gmail.com">
            <Mail className="h-4 w-4" /> profradon@gmail.com
          </a>
          <a className="flex items-center gap-1.5 transition-colors hover:text-primary" href="tel:+2349118932656">
            <Phone className="h-4 w-4" /> 09118932656
          </a>
          <a className="flex items-center gap-1.5 transition-colors hover:text-primary" href="https://wa.me/2349118932656" target="_blank" rel="noreferrer">
            <MessageCircle className="h-4 w-4" /> WhatsApp
          </a>
        </div>
      </div>
    </footer>
  );
}
