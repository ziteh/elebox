<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { dialog } from "@tauri-apps/api";

const target_dir = ref("");
const emit = defineEmits(["notify"]);

function emitNotify(msg: string) {
  emit("notify", { msg });
}

async function handleFileOperation(op: "export" | "import") {
  if (!target_dir.value) {
    return;
  }

  let cmd: string;
  let msg: string;
  if (op === "export") {
    cmd = "export_csv";
    msg = "Export";
  } else {
    cmd = "import_csv";
    msg = "Import";
  }

  try {
    await invoke(cmd, { csv_path: target_dir.value });

    console.log(`${msg} success, ${target_dir.value}`);
    emitNotify(`${msg} success`);
  } catch (err) {
    console.warn(`${msg} fail, ${target_dir.value}, ${err}`);
    emitNotify(`${msg} fail`);
  }
}

async function openDirDialog() {
  const dir = await dialog.open({
    title: "Select Folder for Export or Import",
    directory: true,
    multiple: false,
  });

  if (dir) {
    target_dir.value = Array.isArray(dir) ? dir[0] : dir;
  }

  // TODO move to core
  if (!target_dir.value.endsWith("/")) {
    if (!target_dir.value.endsWith("\\")) {
      target_dir.value += "\\";
    }
  }
}
</script>

<template>
  <v-row class="align-center">
    <v-col>
      <v-text-field
        label="Export/Import Folder"
        variant="outlined"
        v-model="target_dir"
        @click:control="openDirDialog"
      ></v-text-field>
    </v-col>
    <v-col cols="auto" class="mb-6">
      <v-btn @click="handleFileOperation('export')">Export</v-btn>
    </v-col>
    <v-col cols="auto" class="mb-6">
      <v-btn @click="handleFileOperation('import')">Import</v-btn>
    </v-col>
  </v-row>
</template>
