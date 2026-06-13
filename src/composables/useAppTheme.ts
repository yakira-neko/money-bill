import { ref, computed, watch } from "vue";

export type ThemeMode = "light" | "dark";

const STORAGE_KEY = "money-bill-theme";

function getInitialMode(): ThemeMode {
  try {
    const saved = localStorage.getItem(STORAGE_KEY);
    if (saved === "light" || saved === "dark") return saved;
  } catch {
    /* ignore */
  }
  if (typeof window !== "undefined" && window.matchMedia) {
    return window.matchMedia("(prefers-color-scheme: dark)").matches ? "dark" : "light";
  }
  return "light";
}

// Module-level singleton so every consumer shares the same state.
const themeMode = ref<ThemeMode>(getInitialMode());
const isDark = computed(() => themeMode.value === "dark");

function applyMode(mode: ThemeMode) {
  if (typeof document === "undefined") return;
  const root = document.documentElement;
  root.classList.toggle("dark", mode === "dark");
  root.style.colorScheme = mode;
}

// Apply immediately on first import to avoid a flash of the wrong theme.
applyMode(themeMode.value);

watch(themeMode, (mode) => {
  applyMode(mode);
  try {
    localStorage.setItem(STORAGE_KEY, mode);
  } catch {
    /* ignore */
  }
});

function toggleTheme() {
  themeMode.value = themeMode.value === "dark" ? "light" : "dark";
}

function setTheme(mode: ThemeMode) {
  themeMode.value = mode;
}

export function useAppTheme() {
  return { themeMode, isDark, toggleTheme, setTheme };
}
