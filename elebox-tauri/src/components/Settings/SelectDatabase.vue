<script setup lang="ts">
import { onMounted, ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { dialog } from "@tauri-apps/api";

const path = ref("");

const emit = defineEmits(["notify"]);

function emitNotify(msg: string) {
  emit("notify", { msg });
}

async function fetchDbPath() {
  path.value = await invoke("get_db_path", {});
}

async function updateDbPath() {
  if (!path.value) {
    return;
  }

  try {
    await invoke("set_db_path", { new_path: path.value });

    console.log("Database update success");
    emitNotify("Database saved");
  } catch (err) {
    console.warn(`Open database fail, ${err}`);
    emitNotify("Open database fail");
  }
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

onMounted(fetchDbPath);
</script>

<template>
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
      <v-btn @click="updateDbPath">Apply</v-btn>
    </v-col>
  </v-row>
</template>
