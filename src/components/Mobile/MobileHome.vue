<template>
  <div class="p-4 space-y-5">
    <!-- 本月结余 Hero 卡片 -->
    <div class="rounded-3xl p-5 text-white shadow-lg bg-gradient-to-br from-blue-500 to-indigo-600">
      <div class="flex items-center justify-between">
        <span class="text-sm opacity-90">{{ $t('balance') }}</span>
        <span class="text-xs opacity-75">{{ currentMonth }}</span>
      </div>
      <div class="text-4xl font-bold mt-2 tracking-tight">
        {{ formatMoney(income - expenses) }}
      </div>
      <div class="grid grid-cols-2 gap-3 mt-5">
        <div class="rounded-2xl bg-white/15 px-3 py-2 backdrop-blur">
          <div class="flex items-center gap-1 text-xs opacity-90">
            <v-icon icon="mdi-arrow-down-bold" size="14"></v-icon>
            {{ $t('home.income') }}
          </div>
          <div class="text-lg font-semibold mt-0.5">{{ formatMoney(income) }}</div>
        </div>
        <div class="rounded-2xl bg-white/15 px-3 py-2 backdrop-blur">
          <div class="flex items-center gap-1 text-xs opacity-90">
            <v-icon icon="mdi-arrow-up-bold" size="14"></v-icon>
            {{ $t('expenses') }}
          </div>
          <div class="text-lg font-semibold mt-0.5">{{ formatMoney(expenses) }}</div>
        </div>
      </div>
    </div>

    <!-- 本周统计图 -->
    <div>
      <h3 class="text-base font-bold mb-3 text-foreground">{{ $t('home.statistics') }}</h3>
      <ExpensesHistogram />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import ExpensesHistogram from '../PC/Home/ExpensesHistogram.vue'

const income = ref(0)
const expenses = ref(0)

const currentMonth = computed(() => {
  const now = new Date()
  return `${now.getFullYear()}-${String(now.getMonth() + 1).padStart(2, '0')}`
})

const formatMoney = (v: number) =>
  '¥' + (v || 0).toLocaleString(undefined, { minimumFractionDigits: 2, maximumFractionDigits: 2 })

const load = async () => {
  try {
    const res = (await invoke('get_income_expenses')) as number[]
    income.value = res[0] ?? 0
    expenses.value = res[1] ?? 0
  } catch (e) {
    console.error('load income/expenses failed', e)
  }
}

onMounted(load)
</script>
