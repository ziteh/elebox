<script setup lang="ts">
import { onMounted, reactive } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import PartQty from "../components/PartQty.vue";
import PartDel from "../components/PartDel.vue";

interface Parts {
  [index: number]: {
    name: string;
    quantity: number;
    category: string;
    package: string;
    mfr: string;
  };
}

let parts = reactive<Parts>({});

async function getParts() {
  console.log("get parts");
  const ps = await invoke("get_parts", {});
  Object.assign(parts, ps);
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
        :items="Object.values(parts).map((part) => part.name)"
      ></v-autocomplete>
      <v-btn @click="getParts">Update</v-btn>
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
          <td title="12">{{ p.name }}</td>
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
