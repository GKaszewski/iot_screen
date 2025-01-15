import Dashboard from './components/dashboard';
import Navbar from './components/navbar';
import OAuthConfiguration from './components/oauth-configurations';
import OAuthTokenExchanger from './components/oauth-token-exchanger';
import { ThemeProvider } from './components/theme-provider';
import { Toaster } from '@/components/ui/sonner';
import XtbLoginScreen from './components/xtb-login-card';

function App() {
  return (
    <ThemeProvider defaultTheme="dark" storageKey="vite-ui-theme">
      <main className="w-full h-full min-h-screen flex flex-col">
        <Navbar />
        <section className="grid grid-cols-2 w-full h-full p-8">
          <div className="flex flex-col gap-4">
            <OAuthConfiguration />
            <XtbLoginScreen />
          </div>
          <Dashboard />
        </section>
      </main>
      <Toaster />
      <OAuthTokenExchanger />
    </ThemeProvider>
  );
}

export default App;
