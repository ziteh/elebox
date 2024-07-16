<script setup lang="ts">
import { ref, watch } from "vue";
import { CustomField } from "../interface";

const props = defineProps<{
  field_type: string;
  name: string;
  value: string;
  create: Boolean;
}>();

const custom_field = ref<CustomField>({
  name: props.name,
  field_type: props.field_type,
  value: props.value,
});

const emit = defineEmits(["update", "add", "del"]);

watch([custom_field], ([new_custom_field]) => {
  emit("update", { new: new_custom_field });
});

function emitDel() {
  emit("del", { name: props.name });
}

function emitAdd() {
  // Required value
  if (!custom_field.value.field_type || !custom_field.value.name) {
    return;
  }

  emit("add", { new: custom_field.value });
}
</script>

<template>
  <v-select
    label="Type"
    :items="['Normal', 'Link']"
    variant="outlined"
    v-model="custom_field.field_type"
  ></v-select>
  <v-text-field
    label="Name"
    variant="outlined"
    v-model.trim="custom_field.name"
    placeholder=""
  ></v-text-field>
  <v-text-field
    label="Value"
    variant="outlined"
    v-model.trim="custom_field.value"
    placeholder=""
  ></v-text-field>

  <v-btn
    v-if="!props.create"
    density="comfortable"
    icon="mdi-trash-can-outline"
    title="Delete"
    @click="emitDel()"
  ></v-btn>
  <v-btn
    v-if="props.create"
    density="comfortable"
    icon="mdi-plus"
    title="Add"
    color="green"
    @click="emitAdd()"
  ></v-btn>
</template>
