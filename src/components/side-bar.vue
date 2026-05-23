<template>
  <v-navigation-drawer
    v-model="drawerModel"
    :rail="!mobile && rail"
    :permanent="!mobile"
    :temporary="mobile"
    @click="openRail"
    class="!bg-card !border-r !border-border"
    elevation="0"
  >
    <v-list>
      <v-list-item
        :prepend-avatar="rail && !mobile ? undefined : ''"
        class="px-2"
      >
        <template v-slot:prepend v-if="rail && !mobile">
           <v-btn icon="mdi-menu" variant="text" @click.stop="rail = !rail" class="text-foreground"></v-btn>
        </template>
        <template v-slot:append v-if="!rail || mobile">
           <v-btn icon="mdi-chevron-left" variant="text" @click.stop="mobile ? (drawerModel = false) : (rail = !rail)" class="text-foreground"></v-btn>
        </template>
        <v-list-item-title v-if="!rail || mobile" class="text-h6 font-heading font-black text-center text-primary">
            {{ $t("app.name") }}
        </v-list-item-title>
      </v-list-item>
    </v-list>

    <v-divider class="border-border"></v-divider>

    <v-list density="compact" nav class="space-y-1">
      <v-list-item
        prepend-icon="mdi-home"
        :title="$t('home.name')"
        to="/"
        @click="onNavClick"
        active-class="!bg-primary/5 !text-primary"
        class="rounded-lg mb-1"
      ></v-list-item>
      <v-list-item
        prepend-icon="mdi-playlist-plus"
        :title="$t('addTransaction.title')"
        to="/assess"
        @click="onNavClick"
        active-class="!bg-primary/5 !text-primary"
        class="rounded-lg mb-1"
      ></v-list-item>
      <v-list-item
        prepend-icon="mdi-wallet"
        :title="$t('assets.name')"
        to="/assets"
        @click="onNavClick"
        active-class="!bg-primary/5 !text-primary"
        class="rounded-lg mb-1"
      ></v-list-item>
      <v-list-item
        prepend-icon="mdi-chart-pie"
        :title="$t('budget')"
        to="/budget"
        @click="onNavClick"
        active-class="!bg-primary/5 !text-primary"
        class="rounded-lg mb-1"
      ></v-list-item>
      <v-list-item
        prepend-icon="mdi-history"
        :title="$t('history')"
        to="/history"
        @click="onNavClick"
        active-class="!bg-primary/5 !text-primary"
        class="rounded-lg mb-1"
      ></v-list-item>
    </v-list>
  </v-navigation-drawer>
</template>

<script setup lang="ts">
import { ref, computed } from "vue";
import { useDisplay } from "vuetify";

const props = defineProps<{
  modelValue?: boolean;
}>();

const emit = defineEmits<{
  (e: "update:modelValue", value: boolean): void;
}>();

const { mobile } = useDisplay();
const rail = ref(false);

const drawerModel = computed({
  get: () => (mobile.value ? props.modelValue ?? false : true),
  set: (val) => emit("update:modelValue", val),
});

const onNavClick = () => {
  if (mobile.value) {
    emit("update:modelValue", false);
  }
};

const openRail = () => {
  if (!mobile.value) {
    rail.value = false;
  }
};
</script>

