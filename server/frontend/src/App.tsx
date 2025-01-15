import OAuthConfiguration from './components/oauth-configurations';
import OAuthTokenExchanger from './components/oauth-token-exchanger';
import { ThemeProvider } from './components/theme-provider';
import { Toaster } from './components/ui/toaster';

function App() {
  return (
    <ThemeProvider defaultTheme="dark" storageKey="vite-ui-theme">
      <main className="w-full h-full min-h-screen items-center flex flex-col">
        <OAuthConfiguration />
        <OAuthTokenExchanger />
        <Toaster />
      </main>
    </ThemeProvider>
  );
}

export default App;
