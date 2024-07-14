<script setup lang="ts">
import { ref, watch } from "vue";

const props = defineProps<{
  name: string;
  link: string;
  price?: number;
  note: string;
  create: Boolean;
}>();

const l_name = ref(props.name);
const l_link = ref(props.link);
const l_price = ref(props.price);
const l_note = ref(props.note);

const emit = defineEmits(["update", "add", "del"]);

watch(
  [l_name, l_link, l_price, l_note],
  ([new_name, new_link, new_price, new_note]) => {
    emit("update", {
      name: new_name,
      link: new_link,
      price: new_price,
      note: new_note,
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
  if (!l_name.value) {
    return;
  }

  emit("add");
}
</script>

<template>
  <v-text-field
    label="Name"
    variant="outlined"
    v-model="l_name"
    placeholder=""
  ></v-text-field>
  <v-text-field
    label="Link"
    variant="outlined"
    v-model="l_link"
    placeholder="https://"
  ></v-text-field>
  <v-text-field
    label="Price"
    variant="outlined"
    v-model="l_price"
    type="number"
    min="0"
  ></v-text-field>
  <v-text-field
    label="Note"
    variant="outlined"
    v-model="l_note"
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
