<template>
  <v-container class="!p-0">
    <div class="card add-bill-card p-5">
      <h2 class="text-xl font-heading font-bold mb-6 text-foreground">{{$t("addTransaction.title")}}</h2>
      
      <!-- Basic Info -->
      <div class="grid grid-cols-1 md:grid-cols-2 gap-6 mb-6">
        <div>
          <label class="block text-sm font-medium text-muted-foreground mb-2">{{ $t("addBill.dateLabel") }}</label>
          <v-menu
            v-model="dateMenu"
            :close-on-content-click="false"
            transition="scale-transition"
            min-width="auto"
          >
            <template v-slot:activator="{ props }">
              <input
                type="text"
                :value="formattedDate"
                readonly
                v-bind="props"
                class="input-field w-full cursor-pointer"
                :placeholder="$t('addBill.datePlaceholder')"
              />
            </template>
            <v-date-picker
              v-model="date"
              color="primary"
              @update:modelValue="dateMenu = false"
            ></v-date-picker>
          </v-menu>
        </div>
        <div>
          <label class="block text-sm font-medium text-muted-foreground mb-2">{{ $t("addBill.remarkLabel") }}</label>
          <input
            v-model="extra"
            type="text"
            class="input-field w-full"
            :placeholder="$t('addBill.remarkPlaceholder')"
          />
        </div>
      </div>

      <div class="w-full h-px bg-border my-6"></div>

      <!-- Account List -->
      <div class="mb-4 flex justify-between items-center">
        <h3 class="text-base font-medium text-foreground">{{ $t("addBill.accountList") }}</h3>
        <button
          @click="addAccountRow"
          class="btn-primary flex items-center gap-2"
          type="button"
        >
          <v-icon icon="mdi-plus" size="small"></v-icon>
          {{ $t("addBill.addAccount") }}
        </button>
      </div>
      <div class="space-y-4">
        <div v-for="(account, index) in accountList" :key="index" class="account-row">
          <div class="account-row-grid">
            <div class="min-w-0">
               <label class="block text-sm font-medium text-muted-foreground mb-2">
                 {{ $t("addBill.account") }} {{ index + 1 }}
               </label>
               <AddBillAccount
                 :id="'' + index"
                 :displayAccount="account"
                 @changeAccount="changeAccount"
                 class="w-full"
               />
            </div>
            <div class="min-w-0">
               <label class="block text-sm font-medium text-muted-foreground mb-2">{{ $t("addBill.amount") }}</label>
               <div class="relative">
                   <span class="absolute left-3 top-1/2 -translate-y-1/2 text-muted-foreground">¥</span>
                   <input
                     v-model="amounts[index]"
                     type="number"
                     class="bill-control w-full pl-8"
                     :placeholder="$t('addBill.amountPlaceholder')"
                   />
               </div>
            </div>
            <div class="account-row-action">
              <button
                v-if="index > 0"
                @click="deleteAccount(index)"
                class="delete-account-btn"
                :title="$t('addBill.deleteAccountTitle')"
              >
                <v-icon icon="mdi-delete" size="small"></v-icon>
              </button>
            </div>
          </div>
        </div>
      </div>

      <!-- Actions -->
      <div class="mt-8 flex justify-end">
        <button
          @click="addTransaction"
          class="btn-primary flex items-center gap-2"
        >
          <v-icon icon="mdi-check" size="small"></v-icon>
          {{ $t("addBill.addTransaction") }}
        </button>
      </div>
    </div>
  </v-container>
</template>

<script lang="ts" setup>
import { Ref, ref, computed } from "vue";
// import VueDatePicker from "@vuepic/vue-datepicker"; // Removed
// import "@vuepic/vue-datepicker/dist/main.css"; // Removed
import AddBillAccount from "./AddBillAccount.vue";
import { AccountItem } from "./types";
import { invoke } from "@tauri-apps/api/core";
import { useI18n } from 'vue-i18n';
import { format } from 'date-fns';

const { t } = useI18n();

const accountList: Ref<AccountItem[]> = ref([
  {
    name: t("addBillAccount.selectAccount"),
    icon: "/svg/wallet.svg",
  },
]);
const amounts: Ref<string[]> = ref([]);
const date = ref(new Date());
const dateMenu = ref(false);

const formattedDate = computed(() => {
  return date.value ? format(date.value, 'yyyy-MM-dd') : '';
});

const extra = ref("");
const currency = ref("");

const createEmptyAccount = (): AccountItem => ({
  name: t("addBillAccount.selectAccount"),
  icon: "/svg/wallet.svg",
});

const addAccountRow = () => {
  accountList.value.push(createEmptyAccount());
  amounts.value.push("");
};

const changeAccount = (account: AccountItem, index: string) => {
  const idx = parseInt(index);
  accountList.value[idx] = account;
};

const deleteAccount = (index: number) => {
  accountList.value.splice(index, 1);
  amounts.value.splice(index, 1);
};

const addTransaction = () => {
  // 过滤掉未选择的账户
  const validAccounts = accountList.value
    .map((account, index) => ({
      account: account.name,
      amount: parseFloat(amounts.value[index] || "0"),
    }))
    .filter(
      (item) =>
        item.account !== t("addBillAccount.selectAccount") && !isNaN(item.amount) && item.amount !== 0,
    );

  if (validAccounts.length === 0) {
    alert(t("addBill.atLeastOneAccount")); // Could replace with a snackbar later
    return;
  }

  const data = {
    accountAmounts: validAccounts,
    date: Math.floor(date.value.getTime() / 1000),
    extra: extra.value,
    currency: currency.value,
  };

  console.log("添加交易:", data);

  invoke("add_bills", data)
    .then(() => {
      // 重置表单
      accountList.value = [createEmptyAccount()];
      amounts.value = [];
      extra.value = "";
      date.value = new Date();
    })
    .catch((error) => {
      console.error("添加交易失败:", error);
      alert(t("addBill.addTransactionFailed"));
    });
};
</script>

<style scoped>
.add-bill-card {
  border-color: hsl(var(--border) / 0.72);
  box-shadow:
    0 1px 2px rgb(15 23 42 / 0.04),
    0 14px 34px rgb(15 23 42 / 0.06);
}

.account-row {
  padding: 1rem;
  border: 1px solid hsl(var(--border) / 0.72);
  border-radius: var(--radius);
  background:
    linear-gradient(180deg, hsl(var(--card)) 0%, hsl(var(--secondary) / 0.22) 100%);
  box-shadow: inset 0 1px 0 rgb(255 255 255 / 0.75);
}

.account-row-grid {
  display: grid;
  grid-template-columns: minmax(0, 1fr) minmax(240px, 1fr) 44px;
  gap: 1.5rem;
  align-items: end;
}

.bill-control {
  height: 48px;
  min-height: 48px;
  box-sizing: border-box;
  border: 1px solid hsl(var(--border));
  border-radius: var(--radius);
  background-color: hsl(var(--background));
  color: hsl(var(--foreground));
  font: inherit;
  line-height: 1.25rem;
  outline: none;
  transition:
    border-color 150ms ease,
    box-shadow 150ms ease;
}

.bill-control::placeholder {
  color: hsl(var(--muted-foreground));
}

.bill-control:hover {
  border-color: hsl(var(--foreground) / 0.35);
}

.bill-control:focus {
  border-color: hsl(var(--ring));
  box-shadow: 0 0 0 3px hsl(var(--ring) / 0.1);
}

.account-row-action {
  display: flex;
  align-items: center;
  justify-content: center;
  min-height: 50px;
}

.delete-account-btn {
  display: inline-flex;
  width: 36px;
  height: 36px;
  align-items: center;
  justify-content: center;
  border-radius: var(--radius);
  color: hsl(var(--destructive));
  transition:
    background-color 150ms ease,
    box-shadow 150ms ease,
    transform 150ms ease;
}

.delete-account-btn:hover {
  background-color: hsl(var(--destructive) / 0.1);
  box-shadow: inset 0 0 0 1px hsl(var(--destructive) / 0.16);
}

.delete-account-btn:active {
  transform: translateY(1px);
}

@media (max-width: 767px) {
  .account-row-grid {
    grid-template-columns: minmax(0, 1fr);
    gap: 1rem;
    align-items: stretch;
  }

  .account-row-action {
    justify-content: flex-end;
    min-height: auto;
  }
}

/* VueDatePicker overrides to match Vuetify roughly */
:deep(.dp__input) {
  padding: 10px 12px;
  border-radius: 4px;
  border-color: #9e9e9e; /* Grey darken-1 approx */
  font-family: inherit;
}
:deep(.dp__input:hover) {
    border-color: #212121; /* text-color approx */
}
:deep(.dp__input:focus) {
    border-color: #6200ee; /* Primary */
}
</style>
