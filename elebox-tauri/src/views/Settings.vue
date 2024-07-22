<script setup lang="ts">
import { onMounted, ref, reactive } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { useI18n } from "vue-i18n";
import { Config } from "../interface";

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

const config = ref<Config>({});

async function getAssetsPath() {
  assets_path.value = await invoke("get_assets_path", {});
}

let languages = reactive(["en", "zh-Hant"]);
const { locale } = useI18n();

async function changeLanguage() {
  locale.value = config.value.language ?? "en";
  await invoke("set_language", { new_lang: locale.value }); // FIXME cfg type
  // saveConfig();
}

const path = ref("");

async function getPath() {
  path.value = await invoke("get_db_path", {});
  console.debug(`DB path: ${path.value}`);
}

async function setPath() {
  await invoke("set_db_path", { new_path: path.value });
  console.debug(`DB path: ${path.value}`);
}

async function loadLanguage() {
  config.value.language = await invoke("get_language"); // TODO cfg type
}

onMounted(() => {
  getAssetsPath();
  getPath();
  loadLanguage();
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
          v-model="config.language"
          @update:modelValue="changeLanguage"
          label="Select Language"
        ></v-select>
      </v-col>
    </v-row>
    <v-row class="align-center">
      <v-col>
        <v-text-field
          label="Database Path"
          variant="outlined"
          v-model="path"
          placeholder="elebox.db"
        ></v-text-field>
      </v-col>
      <v-col cols="auto" class="mb-6">
        <v-btn @click="setPath" text="Apply"></v-btn>
      </v-col>
    </v-row>
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
        <code>1.0.0-beta.4</code>
      </v-col>
    </v-row>
  </v-container>
</template>
