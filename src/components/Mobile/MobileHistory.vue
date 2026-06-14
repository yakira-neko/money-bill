<template>
  <div class="p-4">
    <h2 class="text-lg font-bold mb-4">{{ $t('history') }}</h2>

    <div v-if="loading" class="flex justify-center py-10">
      <v-progress-circular indeterminate color="primary"></v-progress-circular>
    </div>
    <div v-else-if="transactions.length === 0" class="text-center text-muted-foreground py-16 text-sm">
      {{ $t('no_transactions') }}
    </div>

    <div v-else class="space-y-3">
      <div
        v-for="trans in transactions"
        :key="trans.id"
        class="bg-card text-card-foreground rounded-2xl shadow-sm border border-border/40 overflow-hidden"
      >
        <button class="w-full flex items-center justify-between px-4 py-3" @click="toggle(trans.id)">
          <div class="text-left">
            <div class="font-semibold text-sm">{{ trans.date }}</div>
            <div v-if="trans.extra" class="text-xs text-muted-foreground mt-0.5 truncate max-w-[180px]">
              {{ trans.extra }}
            </div>
          </div>
          <div class="flex items-center gap-2">
            <span
              class="font-bold"
              :class="totalExpenses(trans.details) > 0 ? 'text-rose-600' : 'text-muted-foreground'"
            >
              {{ totalExpenses(trans.details) > 0 ? '-' : '' }}{{ formatAmount(totalExpenses(trans.details)) }}
            </span>
            <v-icon :icon="expanded[trans.id] ? 'mdi-chevron-up' : 'mdi-chevron-down'" size="18"></v-icon>
          </div>
        </button>

        <v-expand-transition>
          <div v-show="expanded[trans.id]" class="px-4 pb-3 border-t border-border/30 pt-2 space-y-1.5">
            <div
              v-for="d in trans.details"
              :key="d.id"
              class="flex items-center justify-between text-sm"
            >
              <span class="text-muted-foreground">{{ displayName(d.account) }}</span>
              <span :class="d.balance >= 0 ? 'text-emerald-600' : 'text-rose-600'" class="font-medium tabular-nums">
                {{ d.balance >= 0 ? '+' : '-' }}{{ formatAmount(Math.abs(d.balance)) }}
              </span>
            </div>
          </div>
        </v-expand-transition>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'

interface Detail { id: string; account: string; balance: number }
interface Transaction { id: string; date: string; extra: string; details: Detail[] }

const transactions = ref<Transaction[]>([])
const loading = ref(true)
const expanded = reactive<Record<string, boolean>>({})

const toggle = (id: string) => (expanded[id] = !expanded[id])

const formatAmount = (v: number) => (v || 0).toFixed(2)

const displayName = (name: string) =>
  name.replace(/^(income|expenses|assets|liabilities)::/, '').replace(/::/g, ' » ')

const totalExpenses = (details: Detail[]) =>
  details.filter((d) => d.account.split('::')[0] === 'expenses').reduce((s, d) => s + d.balance, 0)

const load = async () => {
  loading.value = true
  try {
    const result = (await invoke('get_transaction_history')) as Transaction[]
    transactions.value = result
  } catch (e) {
    console.error('Failed to fetch transactions:', e)
  } finally {
    loading.value = false
  }
}

onMounted(load)
</script>
