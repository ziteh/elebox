<script setup lang="ts">
import { ref } from "vue";
import AppInfo from "@/components/Settings/AppInfo.vue";
import SelectLanguage from "@/components/Settings/SelectLanguage.vue";
import SelectDatabase from "@/components/Settings/SelectDatabase.vue";
import ExImport from "@/components/Settings/ExImport.vue";

const show_notify = ref(false);
const notify_massage = ref("");

function onNotify(data: { msg: string }) {
  show_notify.value = true;
  notify_massage.value = data.msg;
}
</script>

<template>
  <v-container>
    <v-row>
      <h1 class="my-8">{{ $t("settings") }}</h1>
    </v-row>
    <SelectLanguage @notify="onNotify" />
    <SelectDatabase @notify="onNotify" />
    <ExImport @notify="onNotify" />
    <AppInfo />
  </v-container>

  <v-snackbar v-model="show_notify">
    {{ notify_massage }}
    <template v-slot:actions>
      <v-btn
        icon="mdi-close"
        density="comfortable"
        variant="text"
        @click="show_notify = false"
      ></v-btn>
    </template>
  </v-snackbar>
</template>
