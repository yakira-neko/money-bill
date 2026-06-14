<template>
  <div class="flex flex-col w-full h-full p-4 md:p-6 overflow-hidden">
    <div class="flex items-center justify-between mb-4">
      <h1 class="text-2xl font-heading font-bold text-foreground">{{ $t("history") }}</h1>
      <span v-if="transactions.length" class="text-sm text-muted-foreground">
        {{ transactions.length }}
      </span>
    </div>

    <!-- Empty state -->
    <div
      v-if="transactions.length === 0"
      class="flex flex-1 flex-col items-center justify-center text-muted-foreground py-16"
    >
      <v-icon icon="mdi-receipt-text-outline" size="48" class="mb-3 opacity-50"></v-icon>
      <p>{{ $t("no_transactions") }}</p>
    </div>

    <!-- Transaction list -->
    <div v-else class="flex-1 overflow-y-auto space-y-3 pr-1">
      <div
        v-for="transaction in transactions"
        :key="transaction.id"
        class="rounded-lg border border-border bg-card text-card-foreground shadow-sm transition-shadow hover:shadow-md"
      >
        <!-- Header (clickable) -->
        <button
          type="button"
          class="w-full flex items-center justify-between gap-3 px-4 py-3 text-left"
          @click="toggleTransaction(transaction.id)"
        >
          <div class="flex items-center gap-3 min-w-0">
            <div
              class="flex h-9 w-9 items-center justify-center rounded-full bg-primary/10 text-primary shrink-0"
            >
              <v-icon icon="mdi-calendar-blank-outline" size="18"></v-icon>
            </div>
            <div class="min-w-0">
              <div class="font-semibold text-foreground">{{ transaction.date }}</div>
              <div v-if="transaction.extra" class="text-xs text-muted-foreground truncate">
                {{ transaction.extra }}
              </div>
            </div>
          </div>
          <div class="flex items-center gap-2 shrink-0">
            <span
              class="font-bold tabular-nums"
              :class="getTotalExpenses(transaction.details) > 0 ? 'text-destructive' : 'text-muted-foreground'"
            >
              {{ getTotalExpenses(transaction.details) > 0 ? "-" : "" }}{{ formatAmount(getTotalExpenses(transaction.details)) }}
            </span>
            <v-icon
              :icon="collapsedTransactions[transaction.id] ? 'mdi-chevron-down' : 'mdi-chevron-up'"
              class="text-muted-foreground"
            ></v-icon>
          </div>
        </button>

        <!-- Expanded details -->
        <v-expand-transition>
          <div
            v-show="!collapsedTransactions[transaction.id]"
            class="border-t border-border px-4 py-3"
          >
            <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
              <!-- Income column -->
              <div>
                <div class="flex items-center gap-1.5 text-sm font-medium text-success mb-2">
                  <v-icon icon="mdi-arrow-down-bold" size="14"></v-icon>
                  {{ $t("addBillAccount.income") }}
                </div>
                <div
                  v-if="incomeDetails(transaction.details).length === 0"
                  class="text-xs text-muted-foreground"
                >
                  —
                </div>
                <div
                  v-for="detail in incomeDetails(transaction.details)"
                  :key="detail.id"
                  class="flex items-center justify-between gap-2 py-1"
                >
                  <span class="text-sm text-foreground truncate">{{ displayName(detail.account) }}</span>
                  <span class="text-sm font-medium text-success tabular-nums">
                    {{ formatAmount(detail.balance) }}
                  </span>
                </div>
              </div>

              <!-- Expense column -->
              <div class="sm:border-l sm:border-border sm:pl-4">
                <div class="flex items-center gap-1.5 text-sm font-medium text-destructive mb-2">
                  <v-icon icon="mdi-arrow-up-bold" size="14"></v-icon>
                  {{ $t("expenses") }}
                </div>
                <div
                  v-if="expenseDetails(transaction.details).length === 0"
                  class="text-xs text-muted-foreground"
                >
                  —
                </div>
                <div
                  v-for="detail in expenseDetails(transaction.details)"
                  :key="detail.id"
                  class="flex items-center justify-between gap-2 py-1"
                >
                  <span class="text-sm text-foreground truncate">{{ displayName(detail.account) }}</span>
                  <span class="text-sm font-medium text-destructive tabular-nums">
                    {{ formatAmount(Math.abs(detail.balance)) }}
                  </span>
                </div>
              </div>
            </div>
          </div>
        </v-expand-transition>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, reactive } from "vue";
import { invoke } from "@tauri-apps/api/core";

interface TransactionDetail {
  id: string;
  account: string;
  balance: number;
}

interface Transaction {
  id: string;
  date: string;
  extra: string;
  details: TransactionDetail[];
}

const transactions = ref<Transaction[]>([]);
// Object to track collapsed state of transactions
const collapsedTransactions = reactive<Record<string, boolean>>({});

// Fetch transaction history from the backend
const fetchTransactions = async () => {
  try {
    const result: any[] = await invoke("get_transaction_history");
    transactions.value = result.map((item) => ({
      id: item.id,
      date: item.date,
      extra: item.extra,
      details: item.details.map((detail: any) => ({
        id: detail.id,
        account: detail.account,
        balance: detail.balance,
      })),
    }));

    // Initialize all transactions as collapsed
    transactions.value.forEach((transaction) => {
      collapsedTransactions[transaction.id] = true;
    });
  } catch (error) {
    console.error("Failed to fetch transactions:", error);
  }
};

// Toggle transaction collapse state
const toggleTransaction = (id: string) => {
  collapsedTransactions[id] = !collapsedTransactions[id];
};

// Friendly account name (strip type prefix, pretty separators)
const displayName = (name: string) =>
  name.replace(/^(income|expenses|assets|liabilities)::/, "").replace(/::/g, " » ");

const incomeDetails = (details: TransactionDetail[]) =>
  details.filter((detail) => detail.balance > 0);

const expenseDetails = (details: TransactionDetail[]) =>
  details.filter((detail) => detail.balance < 0);

// Format amount with two decimals
const formatAmount = (amount: number) => amount.toFixed(2);

// Calculate total expenses
const getTotalExpenses = (details: TransactionDetail[]) => {
  return details
    .filter((detail) => detail.account.split("::")[0] === "expenses")
    .reduce((sum, detail) => sum - detail.balance, 0);
};

onMounted(() => {
  fetchTransactions();
});
</script>
