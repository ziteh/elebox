<script setup lang="ts">
import { readonly, ref, watch } from "vue";
import { Supplier } from "../interface";

const props = defineProps<{
  name: string;
  link: string;
  price?: number;
  note: string;
  create: Boolean;
}>();

const supplier = ref<Supplier>({
  name: props.name,
  link: props.link,
  note: props.note,
  price: props.price,
});

const emit = defineEmits(["update", "add", "del"]);

watch([supplier], ([new_supplier]) => {
  emit("update", { new: new_supplier });
});

function emitDel() {
  emit("del", { name: props.name });
}

function emitAdd() {
  // Required value
  if (!supplier.value.name) {
    return;
  }

  emit("add", { new: supplier.value });
}
</script>

<template>
  <v-text-field
    label="Name"
    variant="outlined"
    v-model.trim="supplier.name"
    placeholder=""
    :rules="[(v: any) => !!v || 'Required']"
    required
    :readonly="!props.create"
  ></v-text-field>
  <v-text-field
    label="Link"
    variant="outlined"
    v-model.trim="supplier.link"
    placeholder="https://"
    :readonly="!props.create"
  ></v-text-field>
  <v-text-field
    label="Price"
    variant="outlined"
    v-model.number="supplier.price"
    type="number"
    min="0"
    :readonly="!props.create"
  ></v-text-field>
  <v-text-field
    label="Note"
    variant="outlined"
    v-model.trim="supplier.note"
    placeholder=""
    :readonly="!props.create"
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
