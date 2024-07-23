<script setup lang="ts">
import { onMounted, ref, reactive } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { useI18n } from "vue-i18n";
import { Config } from "../types/common";
import { dialog } from "@tauri-apps/api";

const snackbar = ref(false);
const snackbar_msg = ref("");

const ex_import_path = ref("");
const assets_path = ref("");

async function export_csv() {
  await invoke("export_csv", { csv_path: ex_import_path.value });
  console.debug(`Export path: ${ex_import_path.value}`);
}

async function import_csv() {
  await invoke("import_csv", { csv_path: ex_import_path.value });
  console.debug(`Import path: ${ex_import_path.value}`);
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
  snackbar.value = true;
  snackbar_msg.value = "Saved";
}

const path = ref("");

async function getPath() {
  path.value = await invoke("get_db_path", {});
  console.debug(`DB path: ${path.value}`);
}

async function setPath() {
  try {
    await invoke("set_db_path", { new_path: path.value });
  } catch (err) {
    console.warn(`${err}`);
    snackbar.value = true;
    snackbar_msg.value = err as string;
  }
  console.debug(`DB path: ${path.value}`);
  snackbar.value = true;
  snackbar_msg.value = "Saved";
}

async function openDb() {
  const file = await dialog.open({
    title: "Open database",
    directory: false,
    multiple: false,
    filters: [
      {
        name: "Database",
        extensions: ["db"],
      },
      {
        name: "All Files",
        extensions: ["*"],
      },
    ],
  });

  if (file) {
    if (Array.isArray(file)) {
      path.value = file[0];
    } else {
      path.value = file;
    }
  }
}

async function openExImportDir() {
  const file = await dialog.open({
    title: "Export or import",
    directory: true,
    multiple: false,
  });

  if (file) {
    if (Array.isArray(file)) {
      ex_import_path.value = file[0];
    } else {
      ex_import_path.value = file;
    }
  }

  // TODO move to core
  if (!ex_import_path.value.endsWith("/")) {
    if (!ex_import_path.value.endsWith("\\")) {
      ex_import_path.value += "/";
    }
  }
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
          @click:control="openDb"
        ></v-text-field>
      </v-col>
      <v-col cols="auto" class="mb-6">
        <v-btn @click="setPath" text="Apply"></v-btn>
      </v-col>
    </v-row>
    <v-row class="align-center">
      <v-col>
        <v-text-field
          label="Export/Import Path"
          variant="outlined"
          v-model="ex_import_path"
          placeholder="path\to\folder\"
          @click:control="openExImportDir"
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
  <v-snackbar v-model="snackbar">
    {{ snackbar_msg }}
    <template v-slot:actions>
      <v-btn color="pink" variant="text" @click="snackbar = false">
        Close
      </v-btn>
    </template>
  </v-snackbar>
</template>
