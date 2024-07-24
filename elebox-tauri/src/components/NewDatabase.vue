<script setup lang="ts">
import { onMounted, ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { dialog } from "@tauri-apps/api";

const path = ref("");
const default_path = ref("");
const use_default = ref(true);
const empty_db = ref(false);

const emit = defineEmits(["notify"]);

async function getDefaultPath() {
  default_path.value = await invoke("get_default_db_path", {});
  path.value = default_path.value;
}

async function createDb() {
  await invoke("create_db", {
    new_path: use_default ? default_path.value : path.value,
    empty: empty_db.value,
  });
  emit("notify");
}

async function newDb() {
  const file = await dialog.save({
    title: "Create new database",
    defaultPath: default_path.value,
    filters: [
      {
        name: "Database",
        extensions: ["db"],
      },
    ],
  });

  if (file) {
    if (Array.isArray(file)) {
      path.value = file[0];
    } else {
      path.value = file;
    }
    console.log(file);
    use_default.value = false;
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
    use_default.value = false;
    console.log(file);

    await invoke("set_db_path", { new_path: path.value });
    emit("notify");
  }
}

function applyDefaultPath() {
  path.value = default_path.value;
}

onMounted(getDefaultPath);
</script>

<template>
  <v-card
    class="mx-auto"
    prepend-icon="mdi-database-alert"
    title="Database not found"
    subtitle="Chose to create a new database or open an existing one"
  >
    <v-card-item>
      <v-container>
        <v-row class="align-center">
          <v-col cols="auto">
            <v-checkbox
              label="Use default path"
              v-model="use_default"
              hide-details
              @update:modelValue="applyDefaultPath"
            ></v-checkbox>
          </v-col>
          <v-col cols="auto">
            <v-checkbox
              label="Create empty database"
              v-model="empty_db"
              hide-details
              disabled
            ></v-checkbox>
          </v-col>
        </v-row>
        <v-row>
          <v-text-field
            label="Database"
            variant="outlined"
            v-model="path"
            placeholder="elebox.db"
            :disabled="use_default"
            @click:control="newDb"
          ></v-text-field>
        </v-row>
        <v-row>
          <v-spacer></v-spacer>
          <v-col cols="auto">
            <v-btn @click="openDb" text="Open" variant="text"></v-btn>
          </v-col>
          <v-col cols="auto">
            <v-btn @click="createDb" text="Create"></v-btn>
          </v-col>
        </v-row>
      </v-container>
    </v-card-item>
  </v-card>
</template>
