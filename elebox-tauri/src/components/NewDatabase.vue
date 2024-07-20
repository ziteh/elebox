<script setup lang="ts">
import { onMounted, ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { useRouter } from "vue-router";

const path = ref("");
const default_path = ref("");
const use_default = ref(true);

const emit = defineEmits(["update"]);

async function getPath() {
  path.value = await invoke("get_db_path", {});
  console.debug(`DB path: ${path.value}`);
}

async function getDefaultPath() {
  default_path.value = await invoke("get_default_db_path", {});
  //   path.value = default_path.value;
}

async function setPath() {
  await invoke("set_db_path", {
    new_path: use_default ? default_path.value : path.value,
  });
  console.debug(`DB path: ${path.value}`);
  emit("update");
}

onMounted(getDefaultPath);
</script>

<template>
  <v-card
    class="mx-auto"
    prepend-icon="mdi-database-alert"
    title="Database not found"
  >
    <v-card-item>
      <v-container>
        <v-row>
          <v-checkbox
            label="Use default path"
            v-model="use_default"
          ></v-checkbox>
        </v-row>
        <v-row>
          <v-text-field
            v-if="use_default"
            label="Database path"
            variant="outlined"
            v-model="default_path"
            placeholder="elebox.db"
            prepend-icon="mdi-paperclip"
            disabled
          ></v-text-field>
          <v-file-input
            v-else
            label="Database path"
            variant="outlined"
            v-model="path"
            placeholder="elebox.db"
            prepend-icon="mdi-paperclip"
          ></v-file-input>
        </v-row>
        <v-row>
          <v-spacer></v-spacer>
          <v-btn @click="setPath" text="Apply"></v-btn>
        </v-row>
      </v-container>
    </v-card-item>
  </v-card>
</template>
