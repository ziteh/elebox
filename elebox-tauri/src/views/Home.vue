<script setup lang="ts">
import { onMounted, reactive } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import PartQty from "../components/PartQty.vue";
import PartDel from "../components/PartDel.vue";
import { PartData } from "../interface";
import { useRouter, useRoute } from "vue-router";

let parts = reactive<PartData[]>([]);

const router = useRouter();
const route = useRoute();

function reload() {
  // window.location.reload();

  const currentPath = route.path;
  router.replace("/redirect").then(() => {
    router.replace(currentPath);
  });
}

async function updatePage() {
  reload();
}

async function getParts() {
  const ps = await invoke("get_parts", {});
  Object.assign(parts, ps);
  console.log(`get parts ${parts.length}`);
  console.debug(parts);
}

onMounted(getParts);
</script>

<template>
  <v-container>
    <v-row class="ga-8" align="center">
      <v-autocomplete
        label="Search"
        variant="outlined"
        :items="parts.map((part) => part.name)"
      ></v-autocomplete>
      <v-btn @click="updatePage">Update</v-btn>
    </v-row>

    <v-table>
      <thead>
        <tr>
          <th>Part</th>
          <th>Quantity</th>
          <th>Category</th>
          <th>Package</th>
          <th>Mfr</th>
          <th>Edit</th>
        </tr>
      </thead>

      <tbody>
        <tr v-for="(p, index) in parts" :key="index">
          <td>
            <v-btn :to="{ name: 'part_detail', params: { name: p.name } }">
              {{ p.name }}</v-btn
            >
          </td>
          <td>
            {{ p.quantity }}
            <PartQty :part="p.name" />
          </td>
          <td>{{ p.category }}</td>
          <td>{{ p.package }}</td>
          <td>{{ p.mfr }}</td>
          <td>
            <PartDel :part="p.name" />
          </td>
        </tr>
      </tbody>
    </v-table>
  </v-container>
</template>
