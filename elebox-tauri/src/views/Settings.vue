<script setup lang="ts">
import { onMounted, ref, reactive } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import DbPath from "../components/DbPath.vue";
import { useI18n } from "vue-i18n";

const csv_path = ref("");
const assets_path = ref("");

async function export_csv() {
  await invoke("export_csv", { csv_path: csv_path.value });
  console.debug(`Export path: ${csv_path.value}`);
}

async function import_csv() {
  await invoke("import_csv", { csv_path: csv_path.value });
  console.debug(`Import path: ${csv_path.value}`);
}

async function getAssetsPath() {
  assets_path.value = await invoke("get_assets_path", {});
}

let languages = reactive(["en", "zh-TW"]);
const selectedLanguage = ref("en");
const { locale } = useI18n();

function changeLanguage() {
  locale.value = selectedLanguage.value;
  console.log(`Update ${locale.value}`);
}

onMounted(() => {
  getAssetsPath();
  selectedLanguage.value = locale.value;
});
</script>

<template>
  <v-container>
    <v-row>
      <h1 class="my-8">{{ $t("settings") }}</h1>
    </v-row>
    <v-row>
      <v-col>
        <v-select
          variant="outlined"
          :items="languages"
          v-model="selectedLanguage"
          @update:modelValue="changeLanguage"
          label="Select Language"
        ></v-select>
      </v-col>
    </v-row>
    <DbPath />
    <v-row class="align-center">
      <v-col>
        <v-text-field
          label="CSV Path"
          variant="outlined"
          v-model="csv_path"
          placeholder="path\to\folder\"
        ></v-text-field>
      </v-col>
      <v-col cols="auto" class="mb-6">
        <v-btn @click="export_csv">Export</v-btn>
      </v-col>
      <v-col cols="auto" class="mb-6">
        <v-btn @click="import_csv">Import</v-btn>
      </v-col>
    </v-row>
    <v-row class="align-center">
      <v-col cols="auto">
        <v-label>Assets Folder</v-label>
      </v-col>
      <v-col>
        <code>{{ assets_path }}</code>
      </v-col>
    </v-row>
    <v-row class="align-center">
      <!-- <v-spacer></v-spacer> -->
      <v-col cols="auto">
        <v-label>Version</v-label>
      </v-col>
      <v-col cols="auto">
        <code>1.0.0-beta.3</code>
      </v-col>
    </v-row>
  </v-container>
</template>
