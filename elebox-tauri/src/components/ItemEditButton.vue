<script setup lang="ts">
const props = defineProps<{
  item: "category" | "package" | "manufacturer";
  name: string;
}>();
</script>

<template>
  <v-btn
    density="comfortable"
    icon="mdi-square-edit-outline"
    title="Edit"
    :to="{
      name: 'edit',
      params: {
        item: props.item,
        name: props.name,
      },
    }"
  ></v-btn>
</template>
