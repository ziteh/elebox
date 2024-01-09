<script setup lang="ts">
import { onMounted, ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import PartQty from "./PartQty.vue";
import PartDel from "./PartDel.vue";

const parts = ref("");

async function getParts() {
  parts.value = await invoke("get_parts", {});
  console.log(parts.value);
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
        <td>{{ p.part_type }}</td>
        <td>
          <PartDel :part="p.name" />
        </td>
      </tr>
    </tbody>
  </table>
</template>
