<script setup lang="ts">
import { onMounted, reactive } from "vue";
import { DbManufacturer as Db } from "@/utils/db_cmd_manufacturer";
import ManufacturerField from "@/components/Manufacturer/Field.vue";
import ManufacturerList from "@/components/Manufacturer/List.vue";

const existing = reactive<Db.Manufacturer[]>([]);

async function fetchExisting() {
  const data = await Db.list();
  Object.assign(existing, data);

  console.debug(`Get manufacturers: ${existing.length}`);
}

onMounted(fetchExisting);
</script>

<template>
  <v-container>
    <ManufacturerField @update="fetchExisting" />
    <ManufacturerList :existing="existing" @update="fetchExisting" />
  </v-container>
</template>
