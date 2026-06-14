<template>
  <div class="p-4 md:p-6 flex flex-col h-full">
    <div class="flex items-center justify-between mb-4">
      <h1 class="text-2xl font-heading font-bold text-foreground">{{ $t("budgetView.title") }}</h1>
      <button
        @click="openAddDialog"
        class="btn-primary flex items-center gap-2 !py-2 !px-4 !text-sm"
      >
        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4"></path></svg>
        {{ $t("budgetView.addBudget") }}
      </button>
    </div>

    <div class="card flex-1 min-h-[400px]">
      <!-- 加载中 -->
      <div v-if="loading" class="flex justify-center items-center h-64">
        <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-primary"></div>
      </div>

      <!-- 错误 -->
      <div v-else-if="error" class="text-destructive text-center py-8">
        {{ error }}
      </div>

      <!-- 空状态 -->
      <div v-else-if="budgets.length === 0" class="text-center text-muted-foreground py-16">
        {{ $t("budgetView.noBudgets") }}
      </div>

      <!-- 预算列表 -->
      <div v-else class="grid gap-4">
        <div
          v-for="budget in budgets"
          :key="budget.id"
          class="p-4 rounded-lg border border-border bg-card text-card-foreground shadow-sm transition-shadow hover:shadow-md"
        >
          <div class="flex justify-between items-start">
            <div>
              <h3 class="font-semibold text-lg text-foreground">{{ displayName(budget) }}</h3>
              <p class="text-sm text-muted-foreground">{{ $t("budgetView.monthly") }} · {{ currentMonth }}</p>
            </div>
            <div class="flex items-center gap-1">
              <v-btn
                icon="mdi-pencil"
                variant="text"
                size="small"
                color="primary"
                :title="$t('common.edit')"
                @click="openEditDialog(budget)"
              ></v-btn>
              <v-btn
                icon="mdi-delete"
                variant="text"
                size="small"
                color="error"
                :title="$t('common.delete')"
                @click="confirmDelete(budget)"
              ></v-btn>
            </div>
          </div>

          <!-- 金额信息 -->
          <div class="flex justify-between items-baseline mt-3 text-sm">
            <span :class="isOver(budget) ? 'text-destructive font-semibold' : 'text-foreground'">
              {{ $t("budgetView.spent") }}: {{ formatMoney(budget.spent, budget.currency) }}
            </span>
            <span class="text-muted-foreground">
              / {{ formatMoney(budget.amount, budget.currency) }}
            </span>
          </div>

          <!-- 进度条 -->
          <div class="mt-2 w-full bg-muted rounded-full h-2.5 overflow-hidden">
            <div
              class="h-2.5 rounded-full transition-all duration-500"
              :class="progressColor(budget)"
              :style="{ width: progressWidth(budget) + '%' }"
            ></div>
          </div>

          <!-- 剩余 / 超支 -->
          <div class="mt-2 text-right text-sm">
            <span v-if="isOver(budget)" class="text-destructive font-medium">
              {{ $t("budgetView.overBudget") }} {{ formatMoney(budget.spent - budget.amount, budget.currency) }}
            </span>
            <span v-else class="text-success font-medium">
              {{ $t("budgetView.remaining") }} {{ formatMoney(budget.amount - budget.spent, budget.currency) }}
            </span>
          </div>
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
import BudgetDialog from './BudgetDialog.vue'

const { t } = useI18n()

interface Budget {
  id: string
  account: string
  amount: number
  spent: number
  period: string
  currency: string
}

const budgets = ref<Budget[]>([])
const loading = ref(true)
const error = ref<string | null>(null)

const currentMonth = computed(() => {
  const now = new Date()
  return `${now.getFullYear()}-${String(now.getMonth() + 1).padStart(2, '0')}`
})

// 货币符号映射
const currencySymbols: Record<string, string> = {
  CNY: '¥', USD: '$', EUR: '€', JPY: '¥', GBP: '£'
}

const formatMoney = (value: number, currency: string) => {
  const symbol = currencySymbols[currency] || ''
  return `${symbol}${(value || 0).toLocaleString(undefined, { minimumFractionDigits: 2, maximumFractionDigits: 2 })}`
}

const displayName = (budget: Budget) => {
  if (!budget.account || budget.account === 'expenses') {
    return t('budgetView.totalBudget')
  }
  return budget.account.replace(/^expenses::/, '').replace(/::/g, ' » ')
}

const ratio = (budget: Budget) => {
  if (!budget.amount || budget.amount <= 0) return 0
  return budget.spent / budget.amount
}

const isOver = (budget: Budget) => ratio(budget) > 1

const progressWidth = (budget: Budget) => Math.min(ratio(budget) * 100, 100)

const progressColor = (budget: Budget) => {
  const r = ratio(budget)
  if (r > 1) return 'bg-red-500'
  if (r >= 0.8) return 'bg-amber-500'
  return 'bg-blue-500'
}

const loadBudgets = async () => {
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

// 对话框逻辑
const dialogVisible = ref(false)
const isEditMode = ref(false)
const currentBudget = ref<any>({})

const openAddDialog = () => {
  isEditMode.value = false
  currentBudget.value = {}
  dialogVisible.value = true
}

const openEditDialog = (budget: Budget) => {
  isEditMode.value = true
  currentBudget.value = { ...budget }
  dialogVisible.value = true
}

const handleSave = async (item: any) => {
  try {
    if (isEditMode.value) {
      await invoke('update_budget', {
        id: item.id,
        account: item.account,
        amount: item.amount,
        period: item.period,
        currency: item.currency
      })
    } else {
      await invoke('add_budget', {
        account: item.account,
        amount: item.amount,
        period: item.period,
        currency: item.currency
      })
    }
    await loadBudgets()
  } catch (e) {
    console.error('Save budget failed', e)
    alert(t('budgetView.saveError') + ': ' + e)
  }
}

const confirmDelete = async (budget: Budget) => {
  if (confirm(t('budgetView.deleteConfirm'))) {
    try {
      await invoke('delete_budget', { id: budget.id })
      await loadBudgets()
    } catch (e) {
      console.error('Delete budget failed', e)
      alert(t('budgetView.deleteError') + ': ' + e)
    }
  }
}

onMounted(loadBudgets)
</script>

