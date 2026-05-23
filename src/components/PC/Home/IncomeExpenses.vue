<template>
  <div class="grid grid-cols-1 md:grid-cols-3 gap-4 w-full">
    <!-- Income -->
    <div class="metric-card metric-card--income">
      <div class="flex items-center gap-2 mb-2">
        <div class="metric-icon">
          <v-icon icon="mdi-arrow-down-bold" size="small"></v-icon>
        </div>
        <span class="text-sm font-medium text-muted-foreground">{{ $t("home.income") }}</span>
      </div>
      <p class="metric-value">
        {{ income.toFixed(2) }}
      </p>
    </div>

    <!-- Expenses -->
    <div class="metric-card metric-card--expense">
      <div class="flex items-center gap-2 mb-2">
        <div class="metric-icon">
          <v-icon icon="mdi-arrow-up-bold" size="small"></v-icon>
        </div>
        <span class="text-sm font-medium text-muted-foreground">{{ $t("expenses") }}</span>
      </div>
      <p class="metric-value">
        {{ expenses.toFixed(2) }}
      </p>
    </div>

    <!-- Balance -->
    <div class="metric-card metric-card--balance">
      <div class="flex items-center gap-2 mb-2">
        <div class="metric-icon">
          <v-icon icon="mdi-wallet" size="small"></v-icon>
        </div>
        <span class="text-sm font-medium text-muted-foreground">{{ $t("balance") }}</span>
      </div>
      <p class="metric-value">
        {{ (income - expenses).toFixed(2) }}
      </p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

const income = ref(0);
const expenses = ref(0);

// function ch(){
//   i18.changeLocale('en_US')
//   instance!.proxy!.$forceUpdate();
// }
invoke("get_income_expenses").then((i) => {
  income.value = (i as number[])[0];
  expenses.value = (i as number[])[1];
  console.log(i);
});
</script>

<style scoped>
.metric-card {
  --metric-color: hsl(var(--foreground));
  --metric-soft: hsl(var(--muted));

  display: flex;
  min-width: 0;
  flex-direction: column;
  justify-content: space-between;
  border-top: 4px solid var(--metric-color);
  padding: var(--space-md, 1rem);
}

.metric-card--income {
  --metric-color: hsl(var(--metric-income));
  --metric-soft: hsl(var(--metric-income-soft));
}

.metric-card--expense {
  --metric-color: hsl(var(--metric-expense));
  --metric-soft: hsl(var(--metric-expense-soft));
}

.metric-card--balance {
  --metric-color: hsl(var(--metric-balance));
  --metric-soft: hsl(var(--metric-balance-soft));
}

.metric-icon {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 1.75rem;
  height: 1.75rem;
  flex: 0 0 auto;
  border-radius: 9999px;
  background: var(--metric-soft);
  color: var(--metric-color);
}

.metric-value {
  min-width: 0;
  margin-left: var(--space-xs, 0.25rem);
  padding-bottom: var(--space-xs, 0.25rem);
  overflow: hidden;
  color: var(--metric-color);
  font-family: theme("fontFamily.heading");
  font-size: 1.875rem;
  font-weight: 700;
  line-height: 2.25rem;
  text-overflow: ellipsis;
  white-space: nowrap;
}
</style>
