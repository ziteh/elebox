<script setup lang="ts">
import { watch, computed, reactive } from "vue";
import { Supplier } from "@/types/part";

const props = defineProps<{
  current: Supplier;
  existing: string[];
  index?: number;
}>();

const supplier = reactive<Supplier>({
  name: props.current.name ?? "",
  link: props.current.link ?? "",
  note: props.current.note ?? "",
  price: props.current.price,
});

const rules = {
  required: (val: any) => !!val || "Required",
  duplicate: (val: any) =>
    props.existing.every((i, idx) => idx === props.index || i !== val) ||
    "Already exists",
};

const formatted_price = computed({
  get() {
    if (supplier.price == undefined) {
      return "";
    }

    if (props.existing == undefined) {
      return supplier.price.toFixed(3);
    }
    return supplier.price.toString();
  },
  set(value: string) {
    supplier.price = parseFloat(value);
  },
});

const emit = defineEmits(["update", "add", "del"]);

watch([supplier], ([new_supplier]) => {
  console.debug(props.existing);
  emit("update", { new: new_supplier, index: props.index });
});

function emitDel() {
  emit("del", { name: props.current?.name });
}

function emitAdd() {
  // Required value
  if (!supplier.name || rules.duplicate(supplier.name) !== true) {
    return;
  }

  const clone: Supplier = {
    name: supplier.name,
    link: supplier.link ?? "",
    note: supplier.note ?? "",
    price: supplier.price,
  };

  // Clear
  Object.assign(supplier, {
    name: "",
    link: "",
    note: "",
    price: undefined,
  });

  emit("add", { new: clone });
}
</script>

<template>
  <v-form @submit.prevent>
    <v-row class="align-center">
      <v-col cols="2">
        <v-text-field
          label="Name"
          variant="outlined"
          v-model.trim="supplier.name"
          placeholder=""
          :rules="[rules.required, rules.duplicate]"
          required
        ></v-text-field>
      </v-col>
      <v-col cols="3">
        <v-text-field
          label="Link"
          variant="outlined"
          v-model.trim="supplier.link"
          placeholder="https://"
        ></v-text-field>
      </v-col>
      <v-col cols="2">
        <v-text-field
          label="Price"
          variant="outlined"
          v-model="formatted_price"
          type="number"
          min="0"
        ></v-text-field>
      </v-col>
      <v-col>
        <v-text-field
          label="Note"
          variant="outlined"
          v-model.trim="supplier.note"
          placeholder=""
        ></v-text-field>
      </v-col>
      <v-col cols="auto" class="mb-6">
        <v-btn
          v-if="props.index != undefined"
          density="comfortable"
          icon="mdi-trash-can-outline"
          title="Delete"
          type="submit"
          @click="emitDel()"
        ></v-btn>
        <v-btn
          v-else
          density="comfortable"
          icon="mdi-plus"
          title="Add"
          color="green"
          type="submit"
          @click="emitAdd()"
        ></v-btn>
      </v-col>
    </v-row>
  </v-form>
</template>
