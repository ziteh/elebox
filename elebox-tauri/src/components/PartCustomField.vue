<script setup lang="ts">
import { ref, watch } from "vue";

const props = defineProps<{
  field_type: string;
  name: string;
  value: string;
  create: Boolean;
}>();

const l_field_type = ref(props.field_type);
const l_name = ref(props.name);
const l_value = ref(props.value);

const emit = defineEmits(["update", "add", "del"]);

watch(
  [l_field_type, l_name, l_value],
  ([new_field_type, new_name, new_value]) => {
    emit("update", {
      field_type: new_field_type,
      name: new_name,
      value: new_value,
    });
  }
);

function emitDel() {
  emit("del", {
    name: props.name,
  });
}

function emitAdd() {
  // Required value
  if (!l_field_type.value || !l_name.value) {
    return;
  }

  emit("add");
}
</script>

<template>
  <v-select
    label="Type"
    :items="['Normal', 'Link']"
    variant="outlined"
    v-model="l_field_type"
  ></v-select>
  <v-text-field
    label="Name"
    variant="outlined"
    v-model="l_name"
    placeholder=""
  ></v-text-field>
  <v-text-field
    label="Value"
    variant="outlined"
    v-model="l_value"
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
    @click="emitAdd()"
  ></v-btn>
</template>
