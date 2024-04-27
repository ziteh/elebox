<script setup lang="ts">
import { onMounted, reactive } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import PartQty from "./PartQty.vue";
import PartDel from "./PartDel.vue";

interface Parts {
  [index: number]: {
    name: string;
    quantity: number;
    category: string;
  }
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
  <button @click="getParts">Update</button>
  <table>
    <thead>
      <tr>
        <th>Part</th>
        <th>Quantity</th>
        <th>Type</th>
        <th>Edit</th>
      </tr>
    </thead>
    <tbody>
      <tr v-for="(p, index) in parts" :key="index">
        <td>{{ p.name }}</td>
        <td>{{ p.quantity }}
          <PartQty :part="p.name" />
        </td>
        <td>{{ p.category }}</td>
        <td>
          <PartDel :part="p.name" />
        </td>
      </tr>
    </tbody>
  </table>
</template>
