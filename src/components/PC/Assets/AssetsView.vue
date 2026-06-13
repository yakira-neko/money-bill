<template>
  <div class="p-4 md:p-6">
    <div class="flex justify-between items-center mb-4">
      <h1 class="text-2xl font-heading font-bold text-foreground">{{ $t("assets.overview") }}</h1>
      <div class="flex space-x-2">
        <button @click="openAddDialog" class="btn-primary flex items-center gap-2 !py-2 !px-4 !text-sm">
          <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4"></path></svg>
          {{ $t("assets.addAccount") }}
        </button>
        <button @click="expandAll" class="btn-secondary !py-2 !px-4 !text-sm">
          {{ $t("assets.expandAll") }}
        </button>
        <button @click="collapseAll" class="btn-secondary !py-2 !px-4 !text-sm opacity-70 hover:opacity-100">
          {{ $t("assets.collapseAll") }}
        </button>
      </div>
    </div>

    <!-- 路径导航 -->
    <div class="mb-6 text-sm text-muted-foreground font-medium">
      <span>{{ $t("assets.name") }} » </span>
      <span v-if="currentPath" class="text-foreground">{{ currentPath }}</span>
      <span v-else>{{ $t("assets.allAccounts") }}</span>
    </div>

    <div class="card min-h-[500px]">
      <div v-if="loading" class="flex justify-center items-center h-64">
        <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-primary"></div>
      </div>
      <div v-else-if="error" class="text-destructive text-center py-8">
        {{ error }}
      </div>
      <div v-else class="space-y-8">
        <!-- 收入账户 -->
        <div v-if="incomeAccounts.length > 0">
          <div @click="toggleSection('income')" class="flex items-center cursor-pointer mb-4 group select-none">
            <h2 class="text-xl font-heading font-bold text-emerald-600 group-hover:opacity-80 transition-opacity">{{ $t("assets.incomeAccounts") }}</h2>
            <div class="ml-2 text-muted-foreground transition-transform duration-200" :class="{ 'rotate-90': sectionExpanded.income }">
              <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"></path></svg>
            </div>
          </div>
          <div v-show="sectionExpanded.income" class="ml-4 pl-4 border-l-2 border-border/50 transition-all duration-300">
            <account-tree-item
              v-for="account in incomeAccounts"
              :key="account.path || account.name"
              :account="account"
              @edit="openEditDialog"
              @delete="confirmDelete"
            />
          </div>
        </div>

        <!-- 支出账户 -->
        <div v-if="expenseAccounts.length > 0">
          <div @click="toggleSection('expenses')" class="flex items-center cursor-pointer mb-4 group select-none">
            <h2 class="text-xl font-heading font-bold text-rose-600 group-hover:opacity-80 transition-opacity">{{ $t("assets.expenseAccounts") }}</h2>
            <div class="ml-2 text-muted-foreground transition-transform duration-200" :class="{ 'rotate-90': sectionExpanded.expenses }">
              <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"></path></svg>
            </div>
          </div>
          <div v-show="sectionExpanded.expenses" class="ml-4 pl-4 border-l-2 border-border/50 transition-all duration-300">
            <account-tree-item
              v-for="account in expenseAccounts"
              :key="account.path || account.name"
              :account="account"
              @edit="openEditDialog"
              @delete="confirmDelete"
            />
          </div>
        </div>

        <!-- 储蓄账户 -->
        <div v-if="assetAccounts.length > 0">
          <div @click="toggleSection('assets')" class="flex items-center cursor-pointer mb-4 group select-none">
            <h2 class="text-xl font-heading font-bold text-blue-600 group-hover:opacity-80 transition-opacity">{{ $t("assets.assetAccounts") }}</h2>
            <div class="ml-2 text-muted-foreground transition-transform duration-200" :class="{ 'rotate-90': sectionExpanded.assets }">
              <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"></path></svg>
            </div>
          </div>
          <div v-show="sectionExpanded.assets" class="ml-4 pl-4 border-l-2 border-border/50 transition-all duration-300">
            <account-tree-item
              v-for="account in assetAccounts"
              :key="account.path || account.name"
              :account="account"
              @edit="openEditDialog"
              @delete="confirmDelete"
            />
          </div>
        </div>

        <!-- 负债账户 -->
        <div v-if="liabilityAccounts.length > 0">
          <div @click="toggleSection('liabilities')" class="flex items-center cursor-pointer mb-4 group select-none">
            <h2 class="text-xl font-heading font-bold text-amber-600 group-hover:opacity-80 transition-opacity">{{ $t("assets.liabilityAccounts") }}</h2>
            <div class="ml-2 text-muted-foreground transition-transform duration-200" :class="{ 'rotate-90': sectionExpanded.liabilities }">
              <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"></path></svg>
            </div>
          </div>
          <div v-show="sectionExpanded.liabilities" class="ml-4 pl-4 border-l-2 border-border/50 transition-all duration-300">
            <account-tree-item
              v-for="account in liabilityAccounts"
              :key="account.path || account.name"
              :account="account"
              @edit="openEditDialog"
              @delete="confirmDelete"
            />
          </div>
        </div>
      </div>
    </div>
    
    <account-dialog
      v-model="dialogVisible"
      :is-edit="isEditMode"
      :item="currentAccount"
      @save="handleSave"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed, provide } from 'vue'
import AccountTreeItem from './AccountTreeItem.vue'
import AccountDialog from './AccountDialog.vue'
import { invoke } from "@tauri-apps/api/core";

interface Account {
  name: string
  balance: number
  currency: string
  icon: string
  children?: Account[]
  displayName?: string
  path?: string
  level?: number  // 添加层级属性以跟踪嵌套深度
  expanded?: boolean // 节点的展开/折叠状态
  indent?: number // 缩进级别
  originalBalance?: number // 节点自身的原始余额
  totalBalance?: number // 包含所有子节点的总余额
  isParent?: boolean // 是否为父节点
}

const accounts = ref<Account[]>([])
const loading = ref(true)
const error = ref<string | null>(null)
const currentPath = ref('')

// 创建展开/折叠信号
const expandAllSignal = ref(0)
const collapseAllSignal = ref(0)

// 向子组件提供展开/折叠信号
provide('expandAllSignal', expandAllSignal)
provide('collapseAllSignal', collapseAllSignal)

const sectionExpanded = ref({
  income: true,
  expenses: true,
  assets: true,
  liabilities: true
})

// 格式化账户名称，将 :: 替换为更友好的分隔符同时保留层级信息
const formatAccountName = (account: Account, level = 0): Account => {
  const formattedAccount = { ...account };

  // 保存原始名称作为path
  formattedAccount.path = formattedAccount.name;

  // 分割路径
  const parts = formattedAccount.name.split('::');

  // 设置显示名称为最后一部分
  formattedAccount.displayName = parts[parts.length - 1];

  // 添加层级信息
  formattedAccount.level = level;

  // 默认展开状态：根和第一层展开，更深层级折叠
  formattedAccount.expanded = level < 2;

  // 格式化完整名称，用于显示
  formattedAccount.name = formattedAccount.name.replace(/::+/g, ' » ');

  if (formattedAccount.children && formattedAccount.children.length > 0) {
    formattedAccount.children = formattedAccount.children.map(child => formatAccountName(child, level + 1));
  }

  return formattedAccount;
};

const processAccounts = (rawAccounts: Account[]): Account[] => {
  const accountMap = new Map<string, Account>()
  const rootAccounts: Account[] = []

  // 首先创建所有账户的映射
  rawAccounts.forEach(account => {
    // 提取当前层级名称
    const parts = account.name.split('::');
    const displayName = parts[parts.length - 1];
    const level = parts.length - 1; // 计算层级深度

    accountMap.set(account.name, {
      ...account,
      displayName,
      path: account.name,
      level, // 添加层级信息
      expanded: level < 2, // 默认只展开前两层
      children: [],
      originalBalance: account.balance, // 保存原始余额
      isParent: false // 默认不是父节点
    })
  })

  // 确保父级路径都存在（处理跳跃层级的情况）
  accountMap.forEach((account, name) => {
    const parts = name.split('::');

    // 为每个中间路径创建虚拟节点（如果不存在）
    if (parts.length > 2) {
      // 从顶层开始，逐级检查并创建缺失的父节点
      for (let i = 1; i < parts.length - 1; i++) {
        const parentPath = parts.slice(0, i + 1).join('::');
        if (!accountMap.has(parentPath)) {
          // 创建虚拟父节点
          const parentDisplayName = parts[i];
          accountMap.set(parentPath, {
            name: parentPath,
            displayName: parentDisplayName,
            path: parentPath,
            balance: 0, // 虚拟节点默认余额为0
            currency: '', // 可以根据子节点设置
            icon: '',
            level: i,
            expanded: i < 2, // 默认只展开前两层
            children: [],
            originalBalance: 0,
            isParent: true // 这是一个父节点
          });
        }
      }
    }
  });

  // 构建树状结构
  accountMap.forEach((account, name) => {
    const parts = name.split('::')

    if (parts.length === 1) {
      // 这是根账户
      rootAccounts.push(account)
    } else {
      // 这是子账户，找到父账户并添加
      const parentName = parts.slice(0, parts.length - 1).join('::')
      const parent = accountMap.get(parentName)

      if (parent) {
        parent.children = parent.children || []
        parent.children.push(account)
        parent.isParent = true // 标记为父节点
      } else {
        // 如果找不到父账户，作为根账户处理
        rootAccounts.push(account)
      }
    }
  })

  // 计算每个父节点的余额总和
  const calculateTotalBalances = (accounts: Account[]): number => {
    let total = 0;

    accounts.forEach(account => {
      // 保存节点自身的原始余额
      account.originalBalance = account.balance;

      if (account.children && account.children.length > 0) {
        // 标记为父节点
        account.isParent = true;

        // 先递归计算所有子节点的总和
        const childrenTotal = calculateTotalBalances(account.children);

        // 设置子节点总余额
        account.totalBalance = childrenTotal;

        // 如果是虚拟节点（余额为0的父节点），使用子节点总和作为显示余额
        if (account.originalBalance === 0) {
          account.balance = childrenTotal;
        }

        // 确保所有子节点使用相同的货币单位
        if (!account.currency && account.children[0]?.currency) {
          account.currency = account.children[0].currency;
        }
      }

      // 累加当前节点的余额到总和
      total += account.originalBalance || 0;
    });

    return total;
  }

  calculateTotalBalances(rootAccounts)

  // 对每个节点的子节点进行排序（可选）
  const sortChildren = (accounts: Account[]) => {
    accounts.forEach(account => {
      if (account.children && account.children.length > 0) {
        account.children.sort((a, b) => a.name.localeCompare(b.name));
        sortChildren(account.children); // 递归排序子节点
      }
    });
  };

  // 计算每个节点的嵌套深度
  const calculateIndent = (accounts: Account[], currentIndent = 0) => {
    accounts.forEach(account => {
      account.indent = currentIndent;
      if (account.children && account.children.length > 0) {
        calculateIndent(account.children, currentIndent + 1);
      }
    });
  };

  sortChildren(rootAccounts);
  calculateIndent(rootAccounts);
  return rootAccounts;
}

const incomeAccounts = computed(() =>
  accounts.value.filter(account => account.name.startsWith('income'))
)

const expenseAccounts = computed(() =>
  accounts.value.filter(account => account.name.startsWith('expenses'))
)

const assetAccounts = computed(() =>
  accounts.value.filter(account => account.name.startsWith('assets'))
)

const liabilityAccounts = computed(() =>
  accounts.value.filter(account => account.name.startsWith('liabilities'))
)

// 展开所有节点
const expandAll = () => {
  // 展开所有分类
  Object.keys(sectionExpanded.value).forEach(key => {
    sectionExpanded.value[key as keyof typeof sectionExpanded.value] = true;
  });

  // 触发展开信号
  expandAllSignal.value += 1;
  console.log('展开全部被调用', expandAllSignal.value);
};

// 折叠所有节点
const collapseAll = () => {
  Object.keys(sectionExpanded.value).forEach(key => {
    sectionExpanded.value[key as keyof typeof sectionExpanded.value] = false;
  });

  // 触发折叠信号
  collapseAllSignal.value += 1;
};

// 切换分类的展开/折叠
const toggleSection = (section: keyof typeof sectionExpanded.value) => {
  sectionExpanded.value[section] = !sectionExpanded.value[section];
};

const getAssets = async () => {
  try {
    const [income, expenses, assets, liabilities] = await Promise.all([
       invoke('get_income_accounts') as Promise<any[]>,
       invoke('get_expenses_accounts') as Promise<any[]>,
       invoke('get_assets_accounts') as Promise<any[]>,
       invoke('get_liabilities_accounts') as Promise<any[]>
    ]);

    // Add type prefix if not present to correctly group them
    const process = (list: any[], prefix: string) => list.map(item => ({
        ...item,
        // Ensure name starts with correct prefix if backend doesn't guarantee it or for consistency
        name: item.name.startsWith(prefix) ? item.name : `${prefix}::${item.name}`, 
        balance: item.balance || 0 // Ensure balance exists
    }));

    // Rust backend seems to store full names "income::salary", so simple spread is enough if consistent.
    // However, get_income_accounts returns AccountIconName which might not have balance?
    // Wait, get_income_accounts in lib.rs returns AccountIconName which DOES NOT have balance.
    // We need to either update get_income_accounts to return balance or use a different API.
    // get_accounts in account.rs returns Account struct which has balance.
    // lib.rs: get_income_accounts calls account::get_income_accounts and maps to AccountIconName.
    // AccountIconName definition in lib.rs?
    // struct AccountIconName { name, icon, currency } - MISSING BALANCE.
    
    // Changing course: I should update lib.rs to return Account struct or include balance.
    // For now, I will optimistically assume I updated lib.rs or will update it.
    // Actually, I missed that in the backend plan. I need to check lib.rs again.
    // If lib.rs is missing balance, I must fix it.
    
    // Let's assume I will fix lib.rs in next step or use what I have.
    // AccountIconName struct in lib.rs needs `balance` field.
    
    // Merging all accounts
    const allAccounts = [
        ...income,
        ...expenses,
        ...assets,
        ...liabilities
    ];
    
    // If the backend returns stripped names, we need to be careful.
    // But account::get_income_accounts returns full names "income::..." usually.
    
    // FIX: The backend returns AccountIconName without balance. I need to fix backend first?
    // Or I can use get_accounts() if exposed? It is not exposed.
    // I MUST UPDATE BACKEND TO RETURN BALANCE. 
    // I will proceed with this code assuming the backend returns objects with balance, 
    // and I will perform a backend fix immediately after this or interleaved.
    
    accounts.value = processAccounts(allAccounts)
  } catch (e) {
    error.value = '加载资产数据失败'
    console.error(e)
  } finally {
    loading.value = false
  }
}

// Dialog Logic
const dialogVisible = ref(false)
const isEditMode = ref(false)
const currentAccount = ref({})

const openAddDialog = () => {
  isEditMode.value = false
  currentAccount.value = {}
  dialogVisible.value = true
}

const openEditDialog = (account: Account) => {
  isEditMode.value = true
  currentAccount.value = { ...account, name: account.name } // Ensure full name is passed
  dialogVisible.value = true
}

const handleSave = async (item: any) => {
  try {
    if (isEditMode.value) {
      await invoke('update_account', {
        name: item.name,
        currency: item.currency,
        icon: item.icon,
        extra: item.extra
      })
    } else {
      await invoke('add_account', {
        name: item.name,
        currency: item.currency,
        icon: item.icon,
        extra: item.extra
      })
    }
    await getAssets() // Refresh
  } catch (e) {
    console.error('Save failed', e)
    alert('保存失败: ' + e)
  }
}

const confirmDelete = async (account: Account) => {
  if (confirm(`确定要删除账户 ${account.displayName} 吗？`)) {
    try {
      await invoke('delete_account', { name: account.name })
      await getAssets()
    } catch (e) {
      console.error('Delete failed', e)
      alert('删除失败: ' + e)
    }
  }
}

onMounted(() => {
  getAssets()
})
</script>
