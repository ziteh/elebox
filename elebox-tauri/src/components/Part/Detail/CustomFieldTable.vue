<script setup lang="ts">
import { CustomField } from "@/types/part";

const props = defineProps<{
  custom_fields: CustomField[];
}>();
</script>

<template>
  <v-label class="mt-8">{{ $t("custom_fields") }}</v-label>
  <v-table>
    <thead>
      <tr>
        <th>Name</th>
        <th>Value</th>
        <th>Field Type</th>
      </tr>
    </thead>

    <tbody>
      <tr v-for="cf in props.custom_fields" :key="cf.name">
        <td>{{ cf.name }}</td>
        <td>
          <a
            v-if="cf.field_type == 'Link'"
            :href="cf.value"
            target="_blank"
            :title="cf.value"
            ><v-icon> mdi-open-in-new </v-icon>
          </a>
          <div v-else>
            {{ cf.value }}
          </div>
        </td>
        <td>{{ cf.field_type }}</td>
      </tr>
    </tbody>
  </v-table>
</template>
