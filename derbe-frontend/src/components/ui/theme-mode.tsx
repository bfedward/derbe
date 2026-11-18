import {
  createContext,
  type ReactNode,
  useCallback,
  useContext,
  useEffect,
  useMemo,
  useSyncExternalStore,
} from "react";

const THEME_STORAGE_KEY = "derbe-theme";

type ThemeModeContextValue = {
  isDark: boolean;
  toggle: () => void;
};

const ThemeModeContext = createContext<ThemeModeContextValue | null>(null);

function getIsDark(): boolean {
  return (
    typeof document !== "undefined" && document.documentElement.classList.contains("dark")
  );
}

function subscribe(onStoreChange: () => void): () => void {
  if (typeof document === "undefined") return () => {};

  const observer = new MutationObserver(onStoreChange);
  observer.observe(document.documentElement, {
    attributes: true,
    attributeFilter: ["class"],
  });
  return () => observer.disconnect();
}

function getServerIsDark(): boolean {
  return false;
}

function persistTheme(isDark: boolean): void {
  try {
    window.localStorage.setItem(THEME_STORAGE_KEY, isDark ? "dark" : "light");
  } catch {
    // Theme remains available for this session if storage is disabled.
  }
}

export function ThemeModeProvider({ children }: { children: ReactNode }) {
  const isDark = useSyncExternalStore(subscribe, getIsDark, getServerIsDark);

  useEffect(() => {
    let storedTheme: string | null = null;
    try {
      storedTheme = window.localStorage.getItem(THEME_STORAGE_KEY);
    } catch {
      // Fall back to the system preference if storage is disabled.
    }

    const shouldBeDark =
      storedTheme === "dark" ||
      (storedTheme === null && window.matchMedia("(prefers-color-scheme: dark)").matches);

    document.documentElement.classList.toggle("dark", shouldBeDark);
    persistTheme(shouldBeDark);
  }, []);

  const toggle = useCallback(() => {
    const nextIsDark = !getIsDark();
    document.documentElement.classList.toggle("dark", nextIsDark);
    persistTheme(nextIsDark);
  }, []);

  const value = useMemo(() => ({ isDark, toggle }), [isDark, toggle]);

  return <ThemeModeContext.Provider value={value}>{children}</ThemeModeContext.Provider>;
}

export function useThemeMode(): ThemeModeContextValue {
  const context = useContext(ThemeModeContext);
  if (!context) throw new Error("useThemeMode must be used inside ThemeModeProvider");
  return context;
}
