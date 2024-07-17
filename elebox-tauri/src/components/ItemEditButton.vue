<script setup lang="ts">
const props = defineProps<{
  path_name: string;
  item_name: string;
}>();
</script>

<template>
  <v-btn
    density="comfortable"
    icon="mdi-square-edit-outline"
    title="Edit"
    :to="{ name: props.path_name, params: { origin_name: props.item_name } }"
  ></v-btn>
</template>
