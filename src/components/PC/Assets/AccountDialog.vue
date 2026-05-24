<template>
  <v-dialog v-model="isOpen" max-width="500px">
    <v-card>
      <v-card-title>
        <span class="text-h5">{{ isEdit ? $t('assets.editAccount') : $t('assets.addAccount') }}</span>
      </v-card-title>

      <v-card-text>
        <v-container>
          <v-row>
            <v-col cols="12">
              <v-select
                v-if="!isEdit"
                v-model="accountType"
                :items="accountTypes"
                :label="$t('assets.accountType')"
                item-title="label"
                item-value="value"
              ></v-select>
            </v-col>
            <v-col cols="12">
              <v-text-field
                v-model="simpleName"
                :label="$t('assets.accountName')"
                :disabled="isEdit"
                :hint="$t('assets.accountNameHint')"
                persistent-hint
              ></v-text-field>
            </v-col>
            <v-col cols="12">
              <v-select
                v-model="currencyType"
                :items="currencyOptions"
                :label="$t('assets.currency')"
                item-title="label"
                item-value="value"
              ></v-select>
            </v-col>
            <v-col v-if="isCustomCurrency" cols="12" md="7">
              <v-text-field
                v-model="customCurrencyName"
                :label="$t('assets.customCurrencyName')"
                :placeholder="$t('assets.customCurrencyNamePlaceholder')"
              ></v-text-field>
            </v-col>
            <v-col v-if="isCustomCurrency" cols="12" md="5">
              <v-text-field
                v-model="customCurrencySymbol"
                :label="$t('assets.customCurrencySymbol')"
                :placeholder="$t('assets.customCurrencySymbolPlaceholder')"
              ></v-text-field>
            </v-col>
          </v-row>
        </v-container>
      </v-card-text>

      <v-card-actions>
        <v-spacer></v-spacer>
        <v-btn color="blue-darken-1" variant="text" @click="close">
          {{ $t('common.cancel') }}
        </v-btn>
        <v-btn color="blue-darken-1" variant="text" :disabled="!canSave" @click="save">
          {{ $t('common.save') }}
        </v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>

<script setup lang="ts">
import { ref, watch, computed } from 'vue'
import { useI18n } from 'vue-i18n'

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
  name: '',
  currency: 'CNY',
  icon: '',
  extra: ''
})

const simpleName = ref('')
const accountType = ref('assets')
const customCurrencyValue = 'custom'
const currencyType = ref('CNY')
const customCurrencyName = ref('')
const customCurrencySymbol = ref('')

const accountTypes = computed(() => [
  { label: t('addBillAccount.income'), value: 'income' },
  { label: t('addBillAccount.expenses'), value: 'expenses' },
  { label: t('addBillAccount.assets'), value: 'assets' },
  { label: t('addBillAccount.liabilities'), value: 'liabilities' }
])

const currencyOptions = computed(() => [
  { label: t('assets.currencies.CNY'), value: 'CNY' },
  { label: t('assets.currencies.USD'), value: 'USD' },
  { label: t('assets.currencies.EUR'), value: 'EUR' },
  { label: t('assets.currencies.JPY'), value: 'JPY' },
  { label: t('assets.currencies.GBP'), value: 'GBP' },
  { label: t('assets.customCurrency'), value: customCurrencyValue }
])

const isCustomCurrency = computed(() => currencyType.value === customCurrencyValue)
const canSave = computed(() => !isCustomCurrency.value || customCurrencyName.value.trim().length > 0)
const presetCurrencyValues = computed(() =>
  currencyOptions.value
    .filter((currency) => currency.value !== customCurrencyValue)
    .map((currency) => currency.value)
)

const normalizeCustomCurrency = (currency: string) => {
  const trimmedCurrency = currency.trim()
  const symbolMatch = trimmedCurrency.match(/^(\S+)\s+(.+)$/)

  return {
    symbol: symbolMatch ? symbolMatch[1] : '',
    name: symbolMatch ? symbolMatch[2] : trimmedCurrency
  }
}

const resetCurrencyFields = (currency = 'CNY') => {
  if (presetCurrencyValues.value.includes(currency)) {
    currencyType.value = currency
    customCurrencyName.value = ''
    customCurrencySymbol.value = ''
    return
  }

  const customCurrency = normalizeCustomCurrency(currency)
  currencyType.value = customCurrencyValue
  customCurrencyName.value = customCurrency.name
  customCurrencySymbol.value = customCurrency.symbol
}

const buildCurrencyValue = () => {
  if (!isCustomCurrency.value) return currencyType.value

  const name = customCurrencyName.value.trim()
  const symbol = customCurrencySymbol.value.trim()

  return [symbol, name].filter(Boolean).join(' ')
}

watch(() => props.modelValue, (val) => {
  if (val) {
    if (props.isEdit && props.item) {
      editedItem.value = { ...props.item }
      resetCurrencyFields(props.item.currency || 'CNY')
      // Split name to get type and simple name
      const parts = props.item.name.split('::')
      if (parts.length > 0) {
        // First part is likely type if it matches known types, but for nested assets::bank::checking
        // We need to be careful.
        // Actually, for editing, we might just want to let them edit the "rest" of the name?
        // Or just let them edit the display name (last part)?
        // If we rename "assets::bank::checking" to "assets::bank::savings", we just change the last part?
        // But what if they want to move it?
        // For simplicity: just show the full name or handle the suffix.
        // Let's assume we are editing the simple name (last part) and keeping the parent path?
        // This is complex for a simple dialog.
        // Let's simplifying: For Edit, just show Simple Name and assume Type is fixed (or derived from prefix).
        // And we reconstruct it.
        
        // Find the type
        const type = parts[0]
        if (['income', 'expenses', 'assets', 'liabilities'].includes(type)) {
             accountType.value = type
        }
        
        // Simple name is the REST? or just the last part?
        // If name is "assets::bank::checking", simple name is "bank::checking" or just "checking"?
        // If we want to rename the leaf, it's "checking".
        // Let's assume leaf renaming for now.
        simpleName.value = parts[parts.length - 1]
      } else {
        simpleName.value = props.item.name
      }
    } else {
      // Add mode
      editedItem.value = {
        name: '',
        currency: 'CNY',
        icon: '',
        extra: ''
      }
      resetCurrencyFields('CNY')
      simpleName.value = ''
      accountType.value = 'assets'
    }
  }
})

const close = () => {
  isOpen.value = false
}

const save = () => {
    // Construct full name
    // If Edit, we need to preserve parent path if it exists
    let fullName = ''
    
    if (props.isEdit && props.item) {
        const parts = props.item.name.split('::')
        // Replace last part with new simpleName
        if (parts.length > 1) {
            parts[parts.length - 1] = simpleName.value
            fullName = parts.join('::')
        } else {
            // It was just "assets" (unlikely) or "type::name"
            // actually if it was just "foo", then parts.length is 1.
            // But our items usually start with type.
             if (['income', 'expenses', 'assets', 'liabilities'].includes(parts[0])) {
                  // It is likely a root category like "income"? No, "income::salary".
                  // If it is just "income::salary", parts are ["income", "salary"].
                  parts[parts.length - 1] = simpleName.value
                  fullName = parts.join('::')
             } else {
                 fullName = simpleName.value
             }
        }
    } else {
        // Add mode: Type::Name
        fullName = `${accountType.value}::${simpleName.value}`
    }
    
  editedItem.value.name = fullName
  editedItem.value.currency = buildCurrencyValue()
  emit('save', editedItem.value)
  close()
}
</script>
