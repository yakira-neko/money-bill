<template>
  <!-- 顶部应用栏 -->
  <v-app-bar density="comfortable" flat color="primary" class="text-white">
    <v-app-bar-title class="font-bold">{{ currentTitle }}</v-app-bar-title>
    <template #append>
      <v-btn
        variant="text"
        size="small"
        :icon="isDark ? 'mdi-weather-sunny' : 'mdi-weather-night'"
        @click="toggleTheme"
      ></v-btn>
      <v-btn variant="text" size="small" @click="toggleLang">
        {{ locale === 'zh' ? 'EN' : '中' }}
      </v-btn>
    </template>
  </v-app-bar>

  <!-- 内容区 -->
  <v-main class="bg-background">
    <div class="content-scroll">
      <component :is="currentComponent" :key="tab + '-' + refreshKey" />
    </div>
  </v-main>

  <!-- 中央悬浮添加按钮 -->
  <v-btn
    icon="mdi-plus"
    size="large"
    color="primary"
    elevation="8"
    class="mobile-fab"
    @click="showAddBill = true"
  ></v-btn>

  <!-- 底部导航 -->
  <v-bottom-navigation v-model="tab" grow color="primary" :elevation="10" height="64">
    <v-btn value="home">
      <v-icon>mdi-home</v-icon>
      <span class="text-[10px]">{{ $t('home.name') }}</span>
    </v-btn>
    <v-btn value="assets">
      <v-icon>mdi-wallet</v-icon>
      <span class="text-[10px]">{{ $t('assets.name') }}</span>
    </v-btn>
    <!-- 给中央 FAB 预留空间 -->
    <div class="fab-spacer"></div>
    <v-btn value="budget">
      <v-icon>mdi-chart-pie</v-icon>
      <span class="text-[10px]">{{ $t('budget') }}</span>
    </v-btn>
    <v-btn value="history">
      <v-icon>mdi-history</v-icon>
      <span class="text-[10px]">{{ $t('history') }}</span>
    </v-btn>
  </v-bottom-navigation>

  <!-- 全屏记账对话框 -->
  <v-dialog v-model="showAddBill" fullscreen transition="dialog-bottom-transition">
    <v-card class="bg-background">
      <v-toolbar color="primary" density="comfortable">
        <v-btn icon="mdi-close" @click="showAddBill = false"></v-btn>
        <v-toolbar-title>{{ $t('addTransaction.title') }}</v-toolbar-title>
      </v-toolbar>
      <v-card-text class="pa-4">
        <AddBill />
      </v-card-text>
    </v-card>
  </v-dialog>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import { useTheme } from 'vuetify'
import MobileHome from './MobileHome.vue'
import MobileAssets from './MobileAssets.vue'
import MobileBudget from './MobileBudget.vue'
import MobileHistory from './MobileHistory.vue'
import AddBill from '../PC/AddBill/AddBill.vue'
import { useAppTheme } from '../../composables/useAppTheme'

const { t, locale } = useI18n()
const { isDark, toggleTheme } = useAppTheme()

// Sync Vuetify runtime theme with our shared theme state.
const vuetifyTheme = useTheme()
watch(
  isDark,
  (dark) => {
    vuetifyTheme.global.name.value = dark ? 'dark' : 'light'
  },
  { immediate: true }
)

const tab = ref('home')
const showAddBill = ref(false)
const refreshKey = ref(0)

const componentMap: Record<string, any> = {
  home: MobileHome,
  assets: MobileAssets,
  budget: MobileBudget,
  history: MobileHistory
}

const currentComponent = computed(() => componentMap[tab.value] || MobileHome)

const titleMap = computed<Record<string, string>>(() => ({
  home: t('home.name'),
  assets: t('assets.name'),
  budget: t('budget'),
  history: t('history')
}))

const currentTitle = computed(() => titleMap.value[tab.value] || t('app.name'))

const toggleLang = () => {
  locale.value = locale.value === 'zh' ? 'en' : 'zh'
}

// 关闭记账对话框后刷新当前视图（重新挂载触发数据重载）
watch(showAddBill, (val, old) => {
  if (old && !val) refreshKey.value++
})
</script>

<style scoped>
.content-scroll {
  height: 100%;
  overflow-y: auto;
  /* 给底部导航与 FAB 预留空间 */
  padding-bottom: 80px;
}

.mobile-fab {
  position: fixed;
  bottom: 36px;
  left: 50%;
  transform: translateX(-50%);
  z-index: 1010;
}

.fab-spacer {
  width: 72px;
  flex: 0 0 72px;
}
</style>
