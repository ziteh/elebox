<script setup lang="ts">
import { Supplier } from "@/types/part";

const props = defineProps<{
  suppliers: Supplier[];
}>();
</script>

<template>
  <v-label class="mt-8">{{ $t("suppliers") }}</v-label>
  <v-table>
    <thead>
      <tr>
        <th>Name</th>
        <th>Link</th>
        <th>Price</th>
        <th>Note</th>
      </tr>
    </thead>

    <tbody>
      <tr v-for="s in props.suppliers" :key="s.name">
        <td>{{ s.name }}</td>
        <td>
          <a v-if="s.link" :href="s.link" target="_blank" :title="s.link">
            <v-icon> mdi-open-in-new </v-icon>
          </a>
        </td>
        <td>{{ s.price == undefined ? "" : s.price.toFixed(3) }}</td>
        <td>{{ s.note }}</td>
      </tr>
    </tbody>
  </v-table>
</template>
