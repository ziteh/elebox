<script setup lang="ts">
import { onMounted, ref, reactive } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { useI18n } from "vue-i18n";

const { locale } = useI18n();
const language = ref("en");
const languages = reactive([
  { title: "English", value: "en" },
  { title: "繁體中文", value: "zh-Hant" },
]);

const emit = defineEmits(["notify"]);

function emitNotify(msg: string) {
  emit("notify", { msg });
}

async function changeLanguage() {
  locale.value = language.value ?? "en";
  await invoke("set_language", { new_lang: locale.value });
  emitNotify("Language saved");
}

async function fetchCurrentLanguage() {
  language.value = await invoke("get_language");
}

onMounted(fetchCurrentLanguage);
</script>

<template>
  <v-row>
    <v-col>
      <v-select
        variant="outlined"
        :items="languages"
        v-model="language"
        @update:modelValue="changeLanguage"
        label="Select Language"
      ></v-select>
    </v-col>
  </v-row>
</template>
