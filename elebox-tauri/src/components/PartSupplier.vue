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

  const clone: Supplier = {
    name: supplier.value.name,
    link: supplier.value.link ?? "",
    note: supplier.value.note ?? "",
    price: supplier.value.price,
  };

  // Clear
  supplier.value = {
    name: "",
    link: "",
    note: "",
    price: undefined,
  };

  emit("add", { new: clone });
}
</script>

<template>
  <v-form @submit.prevent>
    <v-row class="ma-2 ga-8">
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
        :dirty="!props.create"
      ></v-text-field>
      <v-text-field
        label="Price"
        variant="outlined"
        v-model.number="supplier.price"
        type="number"
        min="0"
        :readonly="!props.create"
        :dirty="!props.create"
      ></v-text-field>
      <v-text-field
        label="Note"
        variant="outlined"
        v-model.trim="supplier.note"
        placeholder=""
        :readonly="!props.create"
        :dirty="!props.create"
      ></v-text-field>

      <v-btn
        v-if="props.create"
        density="comfortable"
        icon="mdi-plus"
        title="Add"
        color="green"
        type="submit"
        @click="emitAdd()"
      ></v-btn>
      <v-btn
        v-else
        density="comfortable"
        icon="mdi-trash-can-outline"
        title="Delete"
        type="submit"
        @click="emitDel()"
      ></v-btn>
    </v-row>
  </v-form>
</template>
