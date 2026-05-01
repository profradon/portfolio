import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { BrowserRouter, Route, Routes } from "react-router-dom";
import { Toaster as Sonner } from "@/components/ui/sonner";
import { Toaster } from "@/components/ui/toaster";
import { TooltipProvider } from "@/components/ui/tooltip";
import { SiteHeader } from "@/components/SiteHeader";
import { SiteFooter } from "@/components/SiteFooter";
import Index from "./pages/Index.tsx";
import NotFound from "./pages/NotFound.tsx";
import Projects from "./pages/Projects.tsx";
import Books from "./pages/Books.tsx";
import Thoughts from "./pages/Thoughts.tsx";
import About from "./pages/About.tsx";
import BlogList from "./pages/BlogList.tsx";
import BlogPost from "./pages/BlogPost.tsx";
import AdminLayout from "./pages/admin/AdminLayout.tsx";
import AdminAbout from "./pages/admin/AdminAbout.tsx";
import AdminProjects from "./pages/admin/AdminProjects.tsx";
import AdminBlogs from "./pages/admin/AdminBlogs.tsx";
import AdminBooks from "./pages/admin/AdminBooks.tsx";
import AdminThoughts from "./pages/admin/AdminThoughts.tsx";

const queryClient = new QueryClient();

const App = () => (
  <QueryClientProvider client={queryClient}>
    <TooltipProvider>
      <Toaster />
      <Sonner />
      <BrowserRouter>
        <SiteHeader />
        <Routes>
          <Route path="/" element={<Index />} />
          <Route path="/projects" element={<Projects />} />
          <Route path="/blog" element={<BlogList />} />
          <Route path="/blog/:slug" element={<BlogPost />} />
          <Route path="/books" element={<Books />} />
          <Route path="/thoughts" element={<Thoughts />} />
          <Route path="/about" element={<About />} />
          <Route path="/admin" element={<AdminLayout />}>
            <Route index element={<AdminAbout />} />
            <Route path="projects" element={<AdminProjects />} />
            <Route path="blogs" element={<AdminBlogs />} />
            <Route path="books" element={<AdminBooks />} />
            <Route path="thoughts" element={<AdminThoughts />} />
          </Route>
          <Route path="*" element={<NotFound />} />
        </Routes>
        <SiteFooter />
      </BrowserRouter>
    </TooltipProvider>
  </QueryClientProvider>
);

export default App;
