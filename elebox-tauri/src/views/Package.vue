<script setup lang="ts">
import { onMounted, reactive } from "vue";
import { DbPackage as Db } from "@/utils/db_cmd_package";
import PackageField from "@/components/Package/Field.vue";
import PackageList from "@/components/Package/List.vue";

const existing = reactive<Db.Package[]>([]);

async function fetchExisting() {
  const data = await Db.list();
  Object.assign(existing, data);

  console.debug(`Get packages: ${existing.length}`);
}

onMounted(fetchExisting);
</script>

<template>
  <v-container>
    <PackageField @update="fetchExisting" />
    <PackageList :existing="existing" @update="fetchExisting" />
  </v-container>
</template>
