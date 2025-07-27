import { Outlet } from 'react-router-dom';
import { Header } from '@/components/layout/Header';

export function MainLayout() {
  return (
    <div className="flex min-h-screen flex-col bg-background w-full">
      <Header />
      <main className="flex-1 w-full py-8">
        <div className="container mx-auto px-4 max-w-6xl">
          <Outlet />
        </div>
      </main>
    </div>
  );
}