<template>
  <v-dialog v-model="isOpen" max-width="500px">
    <v-card>
      <v-card-title>
        <span class="text-h5">{{ isEdit ? $t('budgetView.editBudget') : $t('budgetView.addBudget') }}</span>
      </v-card-title>

      <v-card-text>
        <v-container>
          <v-row>
            <v-col cols="12">
              <v-select
                v-model="editedItem.account"
                :items="categoryItems"
                :label="$t('budgetView.category')"
                :hint="$t('budgetView.categoryHint')"
                persistent-hint
                item-title="label"
                item-value="value"
              ></v-select>
            </v-col>
            <v-col cols="12">
              <v-text-field
                v-model.number="editedItem.amount"
                type="number"
                min="0"
                :label="$t('budgetView.amount')"
                :placeholder="$t('budgetView.amountPlaceholder')"
                :error-messages="amountError ? [$t('budgetView.amountRequired')] : []"
              ></v-text-field>
            </v-col>
            <v-col cols="6">
              <v-select
                v-model="editedItem.period"
                :items="periods"
                :label="$t('budgetView.period')"
                item-title="label"
                item-value="value"
              ></v-select>
            </v-col>
            <v-col cols="6">
              <v-select
                v-model="editedItem.currency"
                :items="currencies"
                :label="$t('budgetView.currency')"
              ></v-select>
            </v-col>
          </v-row>
        </v-container>
      </v-card-text>

      <v-card-actions>
        <v-spacer></v-spacer>
        <v-btn color="blue-darken-1" variant="text" @click="close">
          {{ $t('common.cancel') }}
        </v-btn>
        <v-btn color="blue-darken-1" variant="text" @click="save">
          {{ $t('common.save') }}
        </v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>

<script setup lang="ts">
import { ref, watch, computed, onMounted } from 'vue'
import { useI18n } from 'vue-i18n'
import { invoke } from '@tauri-apps/api/core'

const { t } = useI18n()

const props = defineProps<{
  modelValue: boolean
  item?: any
  isEdit?: boolean
}>()

const emit = defineEmits(['update:modelValue', 'save'])

const isOpen = computed({
  get: () => props.modelValue,
  set: (val) => emit('update:modelValue', val)
})

const editedItem = ref({
  id: '',
  account: '',
  amount: null as number | null,
  period: 'monthly',
  currency: 'CNY'
})

const amountError = ref(false)

// 支出账户列表，作为预算分类可选项；加上「总预算」选项（空字符串）。
const expenseAccounts = ref<{ name: string }[]>([])

const categoryItems = computed(() => {
  const items = [{ label: t('budgetView.totalBudget') + ' (' + t('budgetView.allExpenses') + ')', value: '' }]
  expenseAccounts.value.forEach((acc) => {
    // 显示去掉 "expenses::" 前缀的友好名称
    const display = acc.name.replace(/^expenses::/, '').replace(/::/g, ' » ')
    items.push({ label: display, value: acc.name })
  })
  return items
})

const periods = computed(() => [
  { label: t('budgetView.monthly'), value: 'monthly' }
])

const currencies = ['CNY', 'USD', 'EUR', 'JPY', 'GBP']

const loadExpenseAccounts = async () => {
  try {
    expenseAccounts.value = (await invoke('get_expenses_accounts')) as { name: string }[]
  } catch (e) {
    console.error('Failed to load expense accounts', e)
    expenseAccounts.value = []
  }
}

onMounted(loadExpenseAccounts)

watch(() => props.modelValue, (val) => {
  if (val) {
    amountError.value = false
    loadExpenseAccounts()
    if (props.isEdit && props.item) {
      editedItem.value = {
        id: props.item.id ?? '',
        account: props.item.account ?? '',
        amount: props.item.amount ?? null,
        period: props.item.period || 'monthly',
        currency: props.item.currency || 'CNY'
      }
    } else {
      editedItem.value = {
        id: '',
        account: '',
        amount: null,
        period: 'monthly',
        currency: 'CNY'
      }
    }
  }
})

const close = () => {
  isOpen.value = false
}

const save = () => {
  const amount = Number(editedItem.value.amount)
  if (!amount || amount <= 0 || isNaN(amount)) {
    amountError.value = true
    return
  }
  amountError.value = false
  emit('save', {
    id: editedItem.value.id,
    account: editedItem.value.account,
    amount,
    period: editedItem.value.period,
    currency: editedItem.value.currency
  })
  close()
}
</script>
