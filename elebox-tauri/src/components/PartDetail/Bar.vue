<script setup lang="ts">
import { useRouter } from "vue-router";
import { DbPart as Db } from "@/utils/db_cmd_part";
import ItemEditButton from "@/components/ItemEditButton.vue";
import ItemDeleteButton from "@/components/ItemDeleteButton.vue";

const props = defineProps<{
  name: string;
}>();

const router = useRouter();

async function deleteCurrent() {
  await Db.remove(props.name);
  router.replace("/"); // Back to home
}
</script>

<template>
  <v-app-bar density="compact">
    <template v-slot:prepend>
      <v-btn :to="{ name: 'home' }">Back</v-btn>
    </template>
    <template v-slot:append>
      <ItemEditButton :item="'part'" :name="props.name" />
      <ItemDeleteButton :name="props.name" @delete="deleteCurrent" />
    </template>
  </v-app-bar>
</template>
