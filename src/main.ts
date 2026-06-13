import { createApp } from "vue";
import App from "./App.vue";
import router from "./router.ts";
import "./index.css";
import i18n from "./i18n"; // 引入 i18n 实例
import { useAppTheme } from "./composables/useAppTheme";

// Vuetify
import "vuetify/styles";
import { createVuetify } from "vuetify";
import { aliases, mdi } from "vuetify/iconsets/mdi";
import "@mdi/font/css/materialdesignicons.css";

const { themeMode } = useAppTheme();

// Vuetify themes are kept in sync with the Tailwind design tokens (see index.css)
// so native Vuetify components match the rest of the UI in both light and dark.
const lightColors = {
  background: "#FFFFFF",
  surface: "#FFFFFF",
  "surface-variant": "#F1F5F9",
  "on-surface": "#0F172A",
  "on-background": "#0F172A",
  primary: "#4F46E5",
  secondary: "#64748B",
  error: "#E11D48",
  info: "#3B82F6",
  success: "#059669",
  warning: "#D97706",
};

const darkColors = {
  background: "#0F172A",
  surface: "#1E293B",
  "surface-variant": "#334155",
  "on-surface": "#F8FAFC",
  "on-background": "#F8FAFC",
  primary: "#6366F1",
  secondary: "#94A3B8",
  error: "#F43F5E",
  info: "#60A5FA",
  success: "#10B981",
  warning: "#F59E0B",
};

const vuetify = createVuetify({
  theme: {
    defaultTheme: themeMode.value,
    themes: {
      light: { dark: false, colors: lightColors },
      dark: { dark: true, colors: darkColors },
    },
  },
  icons: {
    defaultSet: "mdi",
    aliases,
    sets: {
      mdi,
    },
  },
});

const app = createApp(App).use(router).use(i18n).use(vuetify);

app.mount("#app");
