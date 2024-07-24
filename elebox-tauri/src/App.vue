<script setup lang="ts">
import { onMounted } from "vue";
import { useI18n } from "vue-i18n";
import { invoke } from "@tauri-apps/api/tauri";
import Sidebar from "@/components/Sidebar.vue";

const { locale } = useI18n();

async function loadLanguage() {
  locale.value = await invoke("get_language"); // TODO cfg type
}

onMounted(loadLanguage);
</script>

<template>
  <v-app>
    <Sidebar />
    <v-main>
      <router-view />
    </v-main>
  </v-app>
</template>
