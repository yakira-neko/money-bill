<script setup lang="ts">
import { ref, watch } from "vue";
import { RouterView } from "vue-router";
import { useDisplay, useTheme } from "vuetify";
import { useI18n } from "vue-i18n";
import SideBar from "./components/side-bar.vue";
import AddBill from "./components/PC/AddBill/AddBill.vue";
import MobileLayout from "./components/Mobile/MobileLayout.vue";
import { useAppTheme } from "./composables/useAppTheme";

const showAddBill = ref(false);
const drawerOpen = ref(false);
const { mobile } = useDisplay();
const { locale } = useI18n();
const { isDark, toggleTheme } = useAppTheme();

// Keep Vuetify's runtime theme in sync with our Tailwind-driven theme state.
const vuetifyTheme = useTheme();
watch(
  isDark,
  (dark) => {
    vuetifyTheme.global.name.value = dark ? "dark" : "light";
  },
  { immediate: true },
);

const toggleLang = () => {
  locale.value = locale.value === "zh" ? "en" : "zh";
};
</script>

<template>
  <v-app>
    <!-- 移动端界面：底部导航 + 专属视图 -->
    <MobileLayout v-if="mobile" />

    <!-- 桌面端界面：侧边栏 + 路由视图 -->
    <template v-else>
      <SideBar v-model="drawerOpen" />

      <v-app-bar density="comfortable" flat border>
        <v-app-bar-title class="font-heading font-bold">{{ $t("app.name") }}</v-app-bar-title>

        <template #append>
          <v-btn
            color="primary"
            variant="flat"
            prepend-icon="mdi-plus"
            class="mr-2 rounded-lg"
            @click="showAddBill = true"
          >
            {{ $t("addTransaction.title") }}
          </v-btn>
          <v-btn
            :icon="isDark ? 'mdi-weather-sunny' : 'mdi-weather-night'"
            variant="text"
            :title="$t('common.toggleTheme')"
            @click="toggleTheme"
          ></v-btn>
          <v-btn variant="text" :title="$t('common.toggleLanguage')" @click="toggleLang">
            {{ locale === "zh" ? "EN" : "中" }}
          </v-btn>
        </template>
      </v-app-bar>

      <v-main class="bg-background min-h-screen">
        <div class="h-full">
          <RouterView />
        </div>
      </v-main>

      <v-dialog v-model="showAddBill" max-width="800" scrollable>
        <v-card class="rounded-lg">
          <v-card-title class="d-flex justify-space-between align-center pa-4">
            <span class="text-h6 font-heading font-bold">{{ $t("addTransaction.title") }}</span>
            <v-btn icon="mdi-close" variant="text" @click="showAddBill = false"></v-btn>
          </v-card-title>
          <v-divider></v-divider>
          <v-card-text class="pa-0">
            <AddBill />
          </v-card-text>
        </v-card>
      </v-dialog>
    </template>
  </v-app>
</template>

<style scoped>
/* Styling handled via Vuetify themes + Tailwind design tokens. */
</style>

