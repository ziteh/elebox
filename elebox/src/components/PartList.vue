<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const parts = ref("");

async function getParts() {
  parts.value = await invoke("get_parts", {});
  console.log(parts.value);
}
</script>

<template>
  <form class="row" @submit.prevent="getParts">
    <!-- <input id="greet-input" v-model="name" placeholder="Enter a name..." /> -->
    <button type="submit">Update</button>
  </form>
  <table>
    <thead>
      <tr>
        <th>Part</th>
        <th>Quantity</th>
        <th>Type</th>
      </tr>
    </thead>
    <tbody>
      <tr v-for="(p, index) in parts" :key="index">
        <td>{{ p.name }}</td>
        <td>{{ p.quantity }}</td>
        <td>{{ p.part_type }}</td>
      </tr>
    </tbody>
  </table>
</template>

