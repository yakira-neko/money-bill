<script setup lang="ts">
import { ref } from "vue";
import { RouterView } from "vue-router";
import SideBar from "./components/side-bar.vue";
import { useDisplay } from "vuetify";

const drawerOpen = ref(false);
const { mobile } = useDisplay();
</script>

<template>
  <v-app>
    <SideBar v-model="drawerOpen" />
    
    <v-app-bar density="compact" flat border>
       <v-app-bar-nav-icon v-if="mobile" @click="drawerOpen = !drawerOpen"></v-app-bar-nav-icon>
       <v-app-bar-title>{{ $t("app.name") }}</v-app-bar-title>
    </v-app-bar>

    <v-main class="bg-background min-h-screen">
      <div class="flex justify-end p-4">
        <v-btn
          to="/assess"
          icon="mdi-plus"
          class="!bg-primary !text-primary-foreground"
          elevation="4"
        ></v-btn>
      </div>
      
      <div class="px-4 pb-4 h-full">
         <RouterView />
      </div>
    </v-main>
  </v-app>
</template>

<style scoped>
/* Custom styles if needed, mostly handled by Vuetify now */
</style>

