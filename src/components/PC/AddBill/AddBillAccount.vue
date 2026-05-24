<template>
  <v-menu v-model="isOpen" :close-on-content-click="false" location="bottom start">
    <template v-slot:activator="{ props }">
      <div
        v-bind="props"
        class="bill-account-trigger"
        @click="isOpen = true"
      >
        <div class="flex items-center gap-2 overflow-hidden">
             <v-avatar size="24" color="grey-lighten-3" class="text-caption text-grey-darken-2 flex-shrink-0">
                 {{ displayAccount.name?.split("::").slice(-1)[0]?.substring(0, 1) || "?" }}
             </v-avatar>
             <span class="truncate text-foreground">
                 {{ displayAccount.name.split('::').slice(-1)[0] || t('addBillAccount.selectAccount') }}
             </span>
        </div>
        <v-icon icon="mdi-chevron-down" color="grey-darken-1" size="small"></v-icon>
      </div>
    </template>

    <v-card width="350">
      <v-tabs v-model="currentType" density="compact" grow color="primary">
        <v-tab value="income">{{ t('addBillAccount.income') }}</v-tab>
        <v-tab value="expenses">{{ t('addBillAccount.expenses') }}</v-tab>
        <v-tab value="assets">{{ t('addBillAccount.assets') }}</v-tab>
        <v-tab value="liabilities">{{ t('addBillAccount.liabilities') }}</v-tab>
      </v-tabs>

      <v-divider></v-divider>

      <v-list density="compact" class="overflow-y-auto" style="max-height: 300px">
        <v-list-item
          v-if="items.length === 0"
          :title="t('addBillAccount.noAccounts')"
          class="text-center text-grey"
        ></v-list-item>
        
        <v-list-item
          v-for="item in items"
          :key="item.name"
          @click="selectAccount(item)"
          block
        >
           <template v-slot:prepend>
              <v-avatar size="24" color="grey-lighten-3" class="text-caption text-grey-darken-2">
                 {{ item.name.split("::").slice(-1)[0]?.substring(0, 1) || "?" }}
              </v-avatar>
           </template>
           <v-list-item-title>{{ item.name.split("::").slice(-1)[0] }}</v-list-item-title>
        </v-list-item>
      </v-list>
    </v-card>
  </v-menu>
</template>

<script setup lang="ts">
import { ref, watchEffect } from "vue";
import { useI18n } from 'vue-i18n';
import type { AccountItem } from "./types";
import { invoke } from "@tauri-apps/api/core";

const { t } = useI18n();

const emit = defineEmits(["changeAccount"]);
const props = defineProps<{
  id: string;
  displayAccount: AccountItem;
}>();

const isOpen = ref(false);
const currentType = ref("income");
const items = ref<AccountItem[]>([]);

const selectAccount = (account: AccountItem) => {
  emit("changeAccount", account, props.id);
  isOpen.value = false;
};

// 获取账户列表
watchEffect(() => {
  if (!isOpen.value) return;

  items.value = [];
  let method = "get_income_accounts";
  
  switch (currentType.value) {
    case "income":
      method = "get_income_accounts";
      break;
    case "expenses":
      method = "get_expenses_accounts";
      break;
    case "assets":
      method = "get_assets_accounts";
      break;
    case "liabilities":
      method = "get_liabilities_accounts";
      break;
  }

  invoke(method)
    .then((res) => {
      const accounts = res as [{ name: string; icon: string }];
      items.value = accounts.map((it) => ({
        icon: it.icon,
        name: it.name,
      }));
    })
    .catch(() => {
      items.value = [];
    });
});
</script>

<style scoped>
.bill-account-trigger {
  display: flex;
  width: 100%;
  height: 48px;
  min-height: 48px;
  box-sizing: border-box;
  cursor: pointer;
  align-items: center;
  justify-content: space-between;
  gap: 0.75rem;
  border: 1px solid hsl(var(--border));
  border-radius: var(--radius);
  background-color: hsl(var(--background));
  padding: 0 0.75rem;
  color: hsl(var(--foreground));
  transition:
    border-color 150ms ease,
    box-shadow 150ms ease;
}

.bill-account-trigger:hover {
  border-color: hsl(var(--foreground) / 0.35);
}

.bill-account-trigger:focus-within {
  border-color: hsl(var(--ring));
  box-shadow: 0 0 0 3px hsl(var(--ring) / 0.1);
}
</style>
