<script setup lang="ts">
import { watch, reactive } from "vue";
import { CustomField } from "@/types/part";

const props = defineProps<{
  current: CustomField;
  existing: string[];
  index?: number;
}>();

const custom_field = reactive<CustomField>({
  field_type: props.current?.field_type ?? "Normal",
  name: props.current?.name ?? "",
  value: props.current?.value ?? "",
});

const rules = {
  required: (val: any) => !!val || "Required",
  duplicate: (val: any) =>
    props.existing.every((i, idx) => idx === props.index || i !== val) ||
    "Already exists",
};

const emit = defineEmits(["update", "add", "del"]);

watch([custom_field], ([new_custom_field]) => {
  console.debug(props.existing);
  emit("update", { new: new_custom_field, index: props.index });
});

function emitDel() {
  emit("del", { name: props.current?.name });
}

function emitAdd() {
  // Required value
  if (
    !custom_field.field_type ||
    !custom_field.name ||
    rules.duplicate(custom_field.name) !== true
  ) {
    return;
  }

  const clone: CustomField = {
    name: custom_field.name,
    field_type: custom_field.field_type,
    value: custom_field.value ?? "",
  };

  // Clear
  Object.assign(custom_field, {
    field_type: "Normal",
    name: "",
    value: "",
  });

  emit("add", { new: clone });
}
</script>

<template>
  <v-form @submit.prevent>
    <v-row class="align-center">
      <v-col cols="2">
        <v-select
          label="Type"
          :items="['Normal', 'Link']"
          variant="outlined"
          v-model="custom_field.field_type"
          :rules="[rules.required]"
          required
        ></v-select>
      </v-col>
      <v-col>
        <v-text-field
          label="Name"
          variant="outlined"
          v-model.trim="custom_field.name"
          :rules="[rules.required, rules.duplicate]"
          required
        ></v-text-field>
      </v-col>
      <v-col>
        <v-text-field
          label="Value"
          variant="outlined"
          v-model.trim="custom_field.value"
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
