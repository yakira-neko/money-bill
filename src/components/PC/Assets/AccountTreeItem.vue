<template>
  <div>
    <div
      class="flex items-center py-1.5 px-1 rounded-md group cursor-pointer select-none transition-colors"
      @click="toggleExpanded"
      :class="{ 'hover:bg-accent': hasChildren, 'parent-node': account.isParent }"
    >
      <div class="flex items-center">
        <span v-if="hasChildren" class="w-5 mr-1 text-muted-foreground inline-flex">
          <v-icon :icon="isExpanded ? 'mdi-chevron-down' : 'mdi-chevron-right'" size="18"></v-icon>
        </span>
        <span v-else class="w-5 mr-1"></span>
        <!-- 名称 -->
        <span class="flex-1 font-medium text-foreground text-left">
          {{ account.displayName || getDisplayName(account.name) }}
        </span>
      </div>
      <!-- 余额、币种等 -->
      <div class="flex ml-auto items-center">
        <div class="mr-4 opacity-0 group-hover:opacity-100 transition-opacity flex space-x-1">
          <button
            class="p-1 rounded text-muted-foreground hover:text-primary hover:bg-accent transition-colors"
            @click.stop="emit('edit', account)"
            :title="$t('common.edit')"
          >
            <v-icon icon="mdi-pencil" size="16"></v-icon>
          </button>
          <button
            class="p-1 rounded text-muted-foreground hover:text-destructive hover:bg-destructive/10 transition-colors"
            @click.stop="emit('delete', account)"
            :title="$t('common.delete')"
          >
            <v-icon icon="mdi-delete" size="16"></v-icon>
          </button>
        </div>
        <span class="w-40 text-right tabular-nums">
          <!-- 父节点显示子节点余额总和，弱化显示 -->
          <template v-if="account.isParent">
            <span class="text-muted-foreground">{{ formatBalance(account.totalBalance || account.balance) }}</span>
          </template>
          <!-- 非父节点显示自身余额 -->
          <template v-else>
            <span class="text-foreground">{{ formatBalance(account.balance) }}</span>
          </template>
        </span>
        <span class="w-12 text-right text-muted-foreground">{{ account.currency }}</span>
      </div>
    </div>
    <div
      v-if="hasChildren && isExpanded"
      class="border-l border-border pl-2"
      :class="{ 'ml-4': getIndentLevel === 0, 'ml-3': getIndentLevel > 0 }"
    >
      <account-tree-item
        v-for="child in account.children"
        :key="child.path || child.name"
        :account="child"
        @edit="emit('edit', $event)"
        @delete="emit('delete', $event)"
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
const emit = defineEmits<{
  (e: 'edit', account: Account): void;
  (e: 'delete', account: Account): void;
}>();

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
    if (props.account) {
      props.account.expanded = true;
    }
  }
});

watch(collapseAllSignal, () => {
  if (collapseAllSignal.value > 0) {
    isExpanded.value = false;
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
  return balance.toLocaleString(undefined, {
    minimumFractionDigits: 2,
    maximumFractionDigits: 2,
    useGrouping: true
  });
};

// Toggle expanded state
const toggleExpanded = () => {
  if (hasChildren.value) {
    isExpanded.value = !isExpanded.value;
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

/* 父节点样式 */
.parent-node {
  position: relative;
  font-weight: 500;
}
.parent-node:hover {
  background-color: hsl(var(--accent));
}
</style>
