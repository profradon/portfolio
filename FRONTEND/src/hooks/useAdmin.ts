import { useEffect, useState } from "react";
import { api } from "@/integrations/client";
import type { User } from "@/integrations/types";

export function useAdmin() {
  const [user, setUser] = useState<User | null>(null);
  const [isAdmin, setIsAdmin] = useState(false);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    checkAuth();
  }, []);

  const checkAuth = async () => {
    const token = localStorage.getItem('auth_token');
    if (!token) {
      setLoading(false);
      return;
    }

    // Set the token in the API client
    api.setAuthToken(token);

    const { data, error } = await api.get<User>('/api/auth/me');
    if (error || !data) {
      localStorage.removeItem('auth_token');
      setLoading(false);
      return;
    }

    setUser(data);
    setIsAdmin(data.role === 'admin');
    setLoading(false);
  };

  const login = async (email: string, password: string) => {
    const { data, error } = await api.post<{ user: User; token: string }>('/api/auth/login', { email, password });
    if (error || !data) {
      throw new Error(error || 'Login failed');
    }

    localStorage.setItem('auth_token', data.token);
    api.setAuthToken(data.token);
    setUser(data.user);
    setIsAdmin(data.user.role === 'admin');
  };

  const signup = async (email: string, password: string) => {
    const { data, error } = await api.post<{ user: User; token: string }>('/api/auth/signup', { email, password });
    if (error || !data) {
      throw new Error(error || 'Signup failed');
    }

    localStorage.setItem('auth_token', data.token);
    api.setAuthToken(data.token);
    setUser(data.user);
    setIsAdmin(data.user.role === 'admin');
  };

  const logout = () => {
    localStorage.removeItem('auth_token');
    api.setAuthToken(null);
    setUser(null);
    setIsAdmin(false);
  };

  return { user, isAdmin, loading, login, signup, logout };
}
