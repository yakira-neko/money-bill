<template>
  <div>
    <div
      class="flex items-center py-1 group cursor-pointer select-none"
      @click="toggleExpanded"
      :class="{'hover:bg-gray-50': hasChildren, 'parent-node': account.isParent}"
    >
      <div class="flex items-center" :style="{ 'margin-left': `${getIndentLevel * 0}px` }">
        <span v-if="hasChildren" class="w-4 mr-1 text-gray-500">
          <svg v-if="isExpanded" class="w-4 h-4 inline" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"/>
          </svg>
          <svg v-else class="w-4 h-4 inline" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"/>
          </svg>
        </span>
        <span v-else class="w-4 mr-1"></span>
        <!-- 名称 -->
        <span class="flex-1 font-medium text-gray-800 text-left">
          {{ account.displayName || getDisplayName(account.name) }}
        </span>
      </div>
      <!-- 余额、币种等 -->
      <div class="flex ml-auto items-center">
        <div class="mr-4 opacity-0 group-hover:opacity-100 transition-opacity flex space-x-2">
           <button class="p-1 hover:bg-gray-200 rounded text-gray-600" @click.stop="$emit('edit', account)" :title="$t('common.edit')">
             <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z"></path></svg>
           </button>
           <button class="p-1 hover:bg-red-100 rounded text-red-600" @click.stop="$emit('delete', account)" :title="$t('common.delete')">
             <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"></path></svg>
           </button>
        </div>
        <span class="w-40 text-right tabular-nums">
          <!-- 父节点显示子节点余额总和，浅灰色显示 -->
          <template v-if="account.isParent">
            <span class="text-gray-400">{{ formatBalance(account.totalBalance || account.balance) }}</span>
          </template>
          <!-- 非父节点显示自身余额 -->
          <template v-else>
            <span class="text-gray-700">{{ formatBalance(account.balance) }}</span>
          </template>
        </span>
        <span
          class="w-20 text-right truncate"
          :title="formatCurrency(account.currency)"
          :class="{'text-gray-400': account.isParent, 'text-gray-600': !account.isParent}"
        >
          {{ formatCurrency(account.currency) }}
        </span>
      </div>
    </div>
    <div v-if="hasChildren && isExpanded"
      class="border-l border-gray-200 pl-2"
      :class="{'ml-4': getIndentLevel === 0, 'ml-3': getIndentLevel > 0, 'border-gray-300': account.isParent}">
      <account-tree-item
        v-for="child in account.children"
        :key="child.path || child.name"
        :account="child"
      />
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
  displayName?: string;
  path?: string;
  level?: number;
  expanded?: boolean;
  indent?: number;
  originalBalance?: number;
  totalBalance?: number;
  isParent?: boolean;
}

const props = defineProps<{ account: Account }>();

// Initialize expanded state based on level, or default to true for root level and first level
// Deeper levels (3+) will be collapsed by default
const isExpanded = ref(props.account.expanded !== undefined ?
  props.account.expanded :
  props.account.level !== undefined ? props.account.level < 2 : true);

const expandAllSignal = inject('expandAllSignal', ref(0));
const collapseAllSignal = inject('collapseAllSignal', ref(0));

watch(expandAllSignal, () => {
  if (expandAllSignal.value > 0) {
    isExpanded.value = true;
    // Update expanded state in account object for persistence
    if (props.account) {
      props.account.expanded = true;
    }
  }
});

watch(collapseAllSignal, () => {
  if (collapseAllSignal.value > 0) {
    isExpanded.value = false;
    // Update expanded state in account object for persistence
    if (props.account) {
      props.account.expanded = false;
    }
  }
});

const hasChildren = computed(() => props.account.children && props.account.children.length > 0);

// Helper function to extract the display name from a full path
const getDisplayName = (name: string) => {
  if (name.includes(' » ')) return name.split(' » ').pop()!;
  if (name.includes('::')) return name.split('::').pop()!;
  return name;
};

const formatBalance = (balance: number) => {
  if (balance == null) return '';
  // 使用固定的小数位数，确保对齐
  return balance.toLocaleString(undefined, {
    minimumFractionDigits: 2,
    maximumFractionDigits: 2,
    useGrouping: true
  });
};

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

// 检查节点是否为父节点且有子节点
const isParentWithChildren = computed(() => {
  return props.account.isParent && hasChildren.value;
});

// Toggle expanded state and propagate to all children if needed
const toggleExpanded = () => {
  if (hasChildren.value) {
    isExpanded.value = !isExpanded.value;
    // Update expanded state in account object for persistence
    if (props.account) {
      props.account.expanded = isExpanded.value;
    }
  }
};

// Helper function to get indentation level
const getIndentLevel = computed(() => {
  return props.account.level || 0;
});
</script>

<style scoped>
.tabular-nums {
  font-variant-numeric: tabular-nums;
}

/* 为不同层级添加不同的视觉效果 */
.level-0 {
  font-weight: 600;
}
.level-1 {
  font-weight: 500;
}
.level-2 {
  font-weight: 400;
}
.level-3, .level-4, .level-5, .level-6 {
  font-weight: 400;
  font-size: 0.95em;
}
.level-7, .level-8, .level-9, .level-10 {
  font-size: 0.9em;
  color: #555;
}

/* 父节点样式 */
.parent-node {
  position: relative;
  font-weight: 500;
}
.parent-node::after {
  content: '';
  position: absolute;
  bottom: -2px;
  left: 0;
  width: 100%;
  height: 1px;
  background-color: #f0f0f0;
}
.parent-node:hover {
  background-color: rgba(0,0,0,0.03);
}
</style>
