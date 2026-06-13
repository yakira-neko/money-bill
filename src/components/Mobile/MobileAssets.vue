<template>
  <div class="p-4 space-y-4">
    <!-- 净资产概览 -->
    <div class="rounded-3xl p-5 text-white shadow-lg bg-gradient-to-br from-emerald-500 to-teal-600">
      <div class="text-sm opacity-90">{{ $t('assets.overview') }}</div>
      <div class="text-4xl font-bold mt-2 tracking-tight">{{ formatMoney(netWorth) }}</div>
      <div class="grid grid-cols-2 gap-3 mt-5">
        <div class="rounded-2xl bg-white/15 px-3 py-2 backdrop-blur">
          <div class="text-xs opacity-90">{{ $t('assets.assetAccounts') }}</div>
          <div class="text-lg font-semibold mt-0.5">{{ formatMoney(totalAssets) }}</div>
        </div>
        <div class="rounded-2xl bg-white/15 px-3 py-2 backdrop-blur">
          <div class="text-xs opacity-90">{{ $t('assets.liabilityAccounts') }}</div>
          <div class="text-lg font-semibold mt-0.5">{{ formatMoney(totalLiabilities) }}</div>
        </div>
      </div>
    </div>

    <div v-if="loading" class="flex justify-center py-10">
      <v-progress-circular indeterminate color="primary"></v-progress-circular>
    </div>
    <div v-else-if="error" class="text-center text-rose-600 py-8">{{ error }}</div>

    <!-- 分组账户列表 -->
    <template v-else>
      <div
        v-for="group in groups"
        :key="group.key"
        class="bg-card text-card-foreground rounded-2xl shadow-sm overflow-hidden border border-border/40"
      >
        <button
          class="w-full flex items-center justify-between px-4 py-3"
          @click="toggle(group.key)"
        >
          <div class="flex items-center gap-2">
            <v-icon :icon="group.icon" :color="group.color" size="20"></v-icon>
            <span class="font-semibold">{{ group.title }}</span>
            <span class="text-xs text-muted-foreground">({{ group.items.length }})</span>
          </div>
          <div class="flex items-center gap-2">
            <span class="font-bold" :class="group.amountClass">{{ formatMoney(group.total) }}</span>
            <v-icon :icon="open[group.key] ? 'mdi-chevron-up' : 'mdi-chevron-down'" size="18"></v-icon>
          </div>
        </button>
        <v-expand-transition>
          <div v-show="open[group.key]">
            <div v-if="group.items.length === 0" class="px-4 pb-3 text-sm text-muted-foreground">
              {{ $t('addBillAccount.noAccounts') }}
            </div>
            <div
              v-for="acc in group.items"
              :key="acc.name"
              class="flex items-center justify-between px-4 py-2.5 border-t border-border/30"
            >
              <span class="text-sm">{{ displayName(acc.name) }}</span>
              <span class="text-sm font-medium tabular-nums">{{ formatMoney(acc.balance) }}</span>
            </div>
          </div>
        </v-expand-transition>
      </div>
    </template>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()

interface Acc { name: string; balance: number; currency: string; icon: string }

const income = ref<Acc[]>([])
const expenses = ref<Acc[]>([])
const assets = ref<Acc[]>([])
const liabilities = ref<Acc[]>([])
const loading = ref(true)
const error = ref<string | null>(null)

const open = reactive<Record<string, boolean>>({
  assets: true,
  liabilities: true,
  income: false,
  expenses: false
})

const toggle = (k: string) => (open[k] = !open[k])

const sum = (list: Acc[]) => list.reduce((s, a) => s + (a.balance || 0), 0)

const totalAssets = computed(() => sum(assets.value))
const totalLiabilities = computed(() => sum(liabilities.value))
const netWorth = computed(() => totalAssets.value - totalLiabilities.value)

const groups = computed(() => [
  { key: 'assets', title: t('assets.assetAccounts'), icon: 'mdi-bank', color: 'blue', items: assets.value, total: totalAssets.value, amountClass: 'text-blue-600' },
  { key: 'liabilities', title: t('assets.liabilityAccounts'), icon: 'mdi-credit-card', color: 'amber', items: liabilities.value, total: totalLiabilities.value, amountClass: 'text-amber-600' },
  { key: 'income', title: t('assets.incomeAccounts'), icon: 'mdi-arrow-down-bold', color: 'green', items: income.value, total: sum(income.value), amountClass: 'text-emerald-600' },
  { key: 'expenses', title: t('assets.expenseAccounts'), icon: 'mdi-arrow-up-bold', color: 'red', items: expenses.value, total: sum(expenses.value), amountClass: 'text-rose-600' }
])

const displayName = (name: string) => name.replace(/^(income|expenses|assets|liabilities)::/, '').replace(/::/g, ' » ')

const formatMoney = (v: number) =>
  '¥' + (v || 0).toLocaleString(undefined, { minimumFractionDigits: 2, maximumFractionDigits: 2 })

const load = async () => {
  loading.value = true
  error.value = null
  try {
    const [inc, exp, ast, lia] = await Promise.all([
      invoke('get_income_accounts') as Promise<Acc[]>,
      invoke('get_expenses_accounts') as Promise<Acc[]>,
      invoke('get_assets_accounts') as Promise<Acc[]>,
      invoke('get_liabilities_accounts') as Promise<Acc[]>
    ])
    income.value = inc
    expenses.value = exp
    assets.value = ast
    liabilities.value = lia
  } catch (e) {
    console.error(e)
    error.value = t('assets.loadError')
  } finally {
    loading.value = false
  }
}

onMounted(load)
</script>
