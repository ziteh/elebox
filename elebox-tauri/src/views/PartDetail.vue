<script setup lang="ts">
import { onMounted, ref } from "vue";
import { useRoute } from "vue-router";
import { DbPart as Db } from "@/utils/db_cmd_part";
import Bar from "@/components/Part/Detail/Bar.vue";
import MainTable from "@/components/Part/Detail/MainTable.vue";
import CustomFieldTable from "@/components/Part/Detail/CustomFieldTable.vue";
import SupplierTable from "@/components/Part/Detail/SupplierTable.vue";

const route = useRoute();

const current = ref<Db.Part>({
  name: "",
  quantity: 0,
  category: "",
  custom_fields: [],
  suppliers: [],
  starred: false,
});

async function fetchCurrent(name: string) {
  const data = await Db.get(name);
  current.value = data as Db.Part;
  console.debug(data);
}

onMounted(() => {
  if (Array.isArray(route.params.name)) {
    fetchCurrent(route.params.name[0]);
  } else {
    fetchCurrent(route.params.name);
  }
});
</script>

<template>
  <v-container v-if="current.name">
    <Bar :name="current.name" />
    <MainTable :current="current" />
    <CustomFieldTable :custom_fields="current.custom_fields" />
    <SupplierTable :suppliers="current.suppliers" />
  </v-container>
</template>
