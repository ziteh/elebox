<script setup lang="ts">
import { onMounted, ref, reactive } from "vue";
import { Part } from "../interface";
import { useRouter, useRoute } from "vue-router";
import { DbPart } from "../db_cmd_part";
import PartQty from "../components/PartQty.vue";
import PartDel from "../components/PartDel.vue";
import ItemEditButton from "../components/ItemEditButton.vue";
import "../styles.css";

let parts = reactive<Part[]>([]);

const router = useRouter();
const route = useRoute();

const search = ref("");
const headers = ref([
  { key: "name", title: "Name", align: "start", sortable: true },
  { key: "quantity", title: "Quantity" },
  { key: "category", title: "Category" },
  { key: "package", title: "Package" },
  { key: "mfr", title: "Manufacturer" },
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
  <v-container>
    <!-- <v-row class="ga-8" align="center">
      <v-autocomplete
        label="Search"
        variant="outlined"
        :items="parts.map((part) => part.name)"
      ></v-autocomplete>
    </v-row> -->

    <v-btn @click="reload">Update</v-btn>

    <v-card flat>
      <template v-slot:text>
        <v-text-field
          v-model="search"
          label="Search"
          prepend-inner-icon="mdi-magnify"
          variant="outlined"
          hide-details
          single-line
        ></v-text-field>
      </template>

      <v-data-table :headers="headers" :items="parts" :search="search">
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
          {{ item.quantity }}
          <PartQty :part="item.name" />
        </template>
      </v-data-table>
    </v-card>
  </v-container>
</template>
