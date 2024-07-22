<script setup lang="ts">
import { onMounted, ref, reactive } from "vue";
import { Part } from "../interface";
import { useRouter, useRoute } from "vue-router";
import { DbPart } from "../db_cmd_part";
import PartQty from "../components/PartQty.vue";
import { useI18n } from "vue-i18n";
const { t } = useI18n();

let parts = reactive<Part[]>([]);

const router = useRouter();
const route = useRoute();

const search = ref("");
const headers = ref([
  { key: "name", title: t("name"), sortable: true },
  { key: "alias", title: t("alias") },
  { key: "quantity", title: t("quantity") },
  { key: "category", title: t("category") },
  { key: "package", title: t("package") },
  { key: "mfr", title: t("mfr") },
]);

async function getParts() {
  let promise = await DbPart.list();
  Object.assign(parts, promise);
}

function reload() {
  // window.location.reload();

  const currentPath = route.path;
  router.replace("/redirect").then(() => {
    router.replace(currentPath);
  });
}

onMounted(getParts);
</script>

<template>
  <v-card flat variant="text">
    <template v-slot:text>
      <v-row class="align-center">
        <v-col>
          <v-text-field
            v-model="search"
            label="Search"
            prepend-inner-icon="mdi-magnify"
            variant="outlined"
            hide-details
            single-line
          ></v-text-field>
        </v-col>
        <v-col cols="auto">
          <v-btn
            @click="reload"
            text="Update"
            icon="mdi-refresh"
            density="comfortable"
            size="large"
            title="Refresh"
          ></v-btn>
        </v-col>
      </v-row>
    </template>

    <v-data-table
      :headers="headers"
      :items="parts"
      :search="search"
      v-if="parts.length > 0"
    >
      <template v-slot:item.name="{ item }">
        <v-btn
          :to="{ name: 'part_detail', params: { name: item.name } }"
          variant="text"
        >
          {{ item.name
          }}<v-icon v-if="item.starred" color="#fcba03">mdi-star</v-icon>
        </v-btn>
      </template>
      <template v-slot:item.quantity="{ item }">
        <PartQty :part="item.name" />
      </template>
    </v-data-table>
  </v-card>
</template>
