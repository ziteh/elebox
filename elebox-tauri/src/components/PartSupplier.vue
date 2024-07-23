<script setup lang="ts">
import { ref, watch, computed } from "vue";
import { Supplier } from "../types/part";
import { makeRules } from "../input_rules";

const props = defineProps<{
  current?: Supplier;
  existing?: Supplier[]; // if undefined: readonly
}>();

const supplier = ref<Supplier>(
  props.current ?? {
    name: "",
    link: "",
    price: undefined,
    note: "",
  }
);

const rules = makeRules(props.existing);

const formatted_price = computed({
  get() {
    if (supplier.value.price === undefined) {
      return "";
    }

    if (props.existing == undefined) {
      return supplier.value.price.toFixed(3);
    }
    return supplier.value.price.toString();
  },
  set(value: string) {
    supplier.value.price = parseFloat(value);
  },
});

const emit = defineEmits(["update", "add", "del"]);

watch([supplier], ([new_supplier]) => {
  emit("update", { new: new_supplier });
});

function emitDel() {
  emit("del", { name: props.current?.name });
}

function emitAdd() {
  // Required value
  if (
    !supplier.value.name ||
    rules.value.duplicate(supplier.value.name) !== true
  ) {
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
    <v-row class="align-center">
      <v-col cols="2">
        <v-text-field
          label="Name"
          variant="outlined"
          v-model.trim="supplier.name"
          placeholder=""
          :rules="[rules.required, rules.duplicate]"
          required
          :readonly="props.existing == undefined"
        ></v-text-field>
      </v-col>
      <v-col cols="3">
        <v-text-field
          label="Link"
          variant="outlined"
          v-model.trim="supplier.link"
          placeholder="https://"
          :readonly="props.existing == undefined"
          :dirty="props.existing == undefined"
        ></v-text-field>
      </v-col>
      <v-col cols="2">
        <v-text-field
          label="Price"
          variant="outlined"
          v-model="formatted_price"
          type="number"
          min="0"
          :readonly="props.existing == undefined"
          :dirty="props.existing == undefined"
        ></v-text-field>
      </v-col>
      <v-col>
        <v-text-field
          label="Note"
          variant="outlined"
          v-model.trim="supplier.note"
          placeholder=""
          :readonly="props.existing == undefined"
          :dirty="props.existing == undefined"
        ></v-text-field>
      </v-col>
      <v-col cols="auto" class="mb-6">
        <v-btn
          v-if="props.existing == undefined"
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
