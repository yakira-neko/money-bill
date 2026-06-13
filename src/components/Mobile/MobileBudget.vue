<template>
  <div class="p-4">
    <div class="flex items-center justify-between mb-4">
      <h2 class="text-lg font-bold">{{ $t('budgetView.title') }}</h2>
      <v-btn
        color="primary"
        size="small"
        variant="tonal"
        prepend-icon="mdi-plus"
        @click="openAdd"
      >{{ $t('budgetView.addBudget') }}</v-btn>
    </div>

    <div v-if="loading" class="flex justify-center py-10">
      <v-progress-circular indeterminate color="primary"></v-progress-circular>
    </div>
    <div v-else-if="error" class="text-center text-rose-600 py-8">{{ error }}</div>
    <div v-else-if="budgets.length === 0" class="text-center text-muted-foreground py-16 text-sm">
      {{ $t('budgetView.noBudgets') }}
    </div>

    <div v-else class="space-y-3">
      <div
        v-for="b in budgets"
        :key="b.id"
        class="bg-card text-card-foreground rounded-2xl shadow-sm border border-border/40 p-4"
        @click="openEdit(b)"
      >
        <div class="flex items-start justify-between">
          <div>
            <div class="font-semibold">{{ displayName(b) }}</div>
            <div class="text-xs text-muted-foreground mt-0.5">{{ $t('budgetView.monthly') }} · {{ currentMonth }}</div>
          </div>
          <v-btn
            icon="mdi-delete-outline"
            size="x-small"
            variant="text"
            color="grey"
            @click.stop="confirmDelete(b)"
          ></v-btn>
        </div>

        <div class="flex items-baseline justify-between mt-3 text-sm">
          <span :class="isOver(b) ? 'text-rose-600 font-semibold' : 'text-foreground'">
            {{ $t('budgetView.spent') }} {{ formatMoney(b.spent, b.currency) }}
          </span>
          <span class="text-muted-foreground">/ {{ formatMoney(b.amount, b.currency) }}</span>
        </div>

        <div class="mt-2 w-full bg-muted rounded-full h-2 overflow-hidden">
          <div
            class="h-2 rounded-full transition-all duration-500"
            :class="progressColor(b)"
            :style="{ width: progressWidth(b) + '%' }"
          ></div>
        </div>

        <div class="mt-1.5 text-right text-xs">
          <span v-if="isOver(b)" class="text-rose-600 font-medium">
            {{ $t('budgetView.overBudget') }} {{ formatMoney(b.spent - b.amount, b.currency) }}
          </span>
          <span v-else class="text-emerald-600">
            {{ $t('budgetView.remaining') }} {{ formatMoney(b.amount - b.spent, b.currency) }}
          </span>
        </div>
      </div>
    </div>

    <budget-dialog
      v-model="dialogVisible"
      :is-edit="isEditMode"
      :item="currentBudget"
      @save="handleSave"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { useI18n } from 'vue-i18n'
import BudgetDialog from '../PC/Budget/BudgetDialog.vue'

const { t } = useI18n()

interface Budget { id: string; account: string; amount: number; spent: number; period: string; currency: string }

const budgets = ref<Budget[]>([])
const loading = ref(true)
const error = ref<string | null>(null)

const currentMonth = computed(() => {
  const now = new Date()
  return `${now.getFullYear()}-${String(now.getMonth() + 1).padStart(2, '0')}`
})

const currencySymbols: Record<string, string> = { CNY: '¥', USD: '$', EUR: '€', JPY: '¥', GBP: '£' }
const formatMoney = (v: number, c: string) =>
  (currencySymbols[c] || '') + (v || 0).toLocaleString(undefined, { minimumFractionDigits: 2, maximumFractionDigits: 2 })

const displayName = (b: Budget) =>
  !b.account || b.account === 'expenses'
    ? t('budgetView.totalBudget')
    : b.account.replace(/^expenses::/, '').replace(/::/g, ' » ')

const ratio = (b: Budget) => (!b.amount || b.amount <= 0 ? 0 : b.spent / b.amount)
const isOver = (b: Budget) => ratio(b) > 1
const progressWidth = (b: Budget) => Math.min(ratio(b) * 100, 100)
const progressColor = (b: Budget) => {
  const r = ratio(b)
  if (r > 1) return 'bg-red-500'
  if (r >= 0.8) return 'bg-amber-500'
  return 'bg-blue-500'
}

const load = async () => {
  loading.value = true
  error.value = null
  try {
    budgets.value = (await invoke('get_budgets')) as Budget[]
  } catch (e) {
    console.error(e)
    error.value = t('budgetView.loadError')
  } finally {
    loading.value = false
  }
}

const dialogVisible = ref(false)
const isEditMode = ref(false)
const currentBudget = ref<any>({})

const openAdd = () => {
  isEditMode.value = false
  currentBudget.value = {}
  dialogVisible.value = true
}
const openEdit = (b: Budget) => {
  isEditMode.value = true
  currentBudget.value = { ...b }
  dialogVisible.value = true
}

const handleSave = async (item: any) => {
  try {
    if (isEditMode.value) {
      await invoke('update_budget', { id: item.id, account: item.account, amount: item.amount, period: item.period, currency: item.currency })
    } else {
      await invoke('add_budget', { account: item.account, amount: item.amount, period: item.period, currency: item.currency })
    }
    await load()
  } catch (e) {
    console.error('save budget failed', e)
    alert(t('budgetView.saveError') + ': ' + e)
  }
}

const confirmDelete = async (b: Budget) => {
  if (confirm(t('budgetView.deleteConfirm'))) {
    try {
      await invoke('delete_budget', { id: b.id })
      await load()
    } catch (e) {
      console.error('delete budget failed', e)
      alert(t('budgetView.deleteError') + ': ' + e)
    }
  }
}

onMounted(load)
</script>
