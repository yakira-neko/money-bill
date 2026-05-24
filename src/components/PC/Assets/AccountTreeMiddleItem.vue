<template>
  <div class="mb-2">
    <div 
      class="flex items-center p-3 bg-gray-50 rounded-lg hover:bg-gray-100 transition-colors cursor-pointer"
      @click="toggleExpanded"
    >
      <div v-if="hasChildren" class="mr-2 text-gray-500 w-4">
        <svg v-if="isExpanded" class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"></path>
        </svg>
        <svg v-else class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"></path>
        </svg>
      </div>
      <div v-else class="w-4 mr-2"></div>
      <img :src="account.icon" :alt="displayName" class="w-8 h-8 mr-3" onerror="this.src='/icons/default.png'" />
      <div class="flex-1">
        <div class="font-medium">{{ displayName }}</div>
        <div class="text-sm text-gray-500">
          {{ account.balance }} {{ formatCurrency(account.currency) }}
        </div>
      </div>
    </div>
    <div v-if="hasChildren && isExpanded" class="ml-6 mt-2 pl-2 border-l-2 border-gray-200">
      <div v-for="child in children" :key="child.name" class="mb-2">
        <!-- 直接显示子项 -->
        <div 
          class="flex items-center p-3 bg-gray-50 rounded-lg hover:bg-gray-100 transition-colors cursor-pointer"
        >
          <div class="w-4 mr-2"></div>
          <img :src="child.icon" :alt="child.name" class="w-8 h-8 mr-3" onerror="this.src='/icons/default.png'" />
          <div class="flex-1">
            <div class="font-medium">{{ child.name }}</div>
            <div class="text-sm text-gray-500">
              {{ child.balance }} {{ formatCurrency(child.currency) }}
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, ref, inject, watch } from 'vue';

interface Account {
  name: string;
  balance: number;
  currency: string;
  icon: string;
  children?: Account[];
}

const props = defineProps<{
  account: Account;
  level: string;
  children: Account[];
}>();

// 控制展开/收起状态
const isExpanded = ref(true);

// 注入全局展开/折叠状态
const expandAllSignal = inject('expandAllSignal', ref(0));
const collapseAllSignal = inject('collapseAllSignal', ref(0));

// 监听展开信号
watch(expandAllSignal, () => {
  if (expandAllSignal.value > 0) {
    isExpanded.value = true;
  }
});

// 监听折叠信号
watch(collapseAllSignal, () => {
  if (collapseAllSignal.value > 0) {
    isExpanded.value = false;
  }
});

// 判断是否有子节点
const hasChildren = computed(() => {
  return props.children && props.children.length > 0;
});

// 显示名称
const displayName = computed(() => {
  return props.level;
});

const formatCurrency = (currency: string) => {
  const currencySymbols: Record<string, string> = {
    CNY: '¥',
    USD: '$',
    EUR: '€',
    JPY: '¥',
    GBP: '£'
  };

  return currencySymbols[currency] ? `${currencySymbols[currency]} ${currency}` : currency;
};

// 切换展开/收起状态
const toggleExpanded = () => {
  if (hasChildren.value) {
    isExpanded.value = !isExpanded.value;
  }
};
</script>

<style scoped>
.cursor-pointer {
  cursor: pointer;
}
</style> 
