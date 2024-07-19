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
  field_type: props.field_type || "Normal",
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

  const clone: CustomField = {
    name: custom_field.value.name,
    field_type: custom_field.value.field_type,
    value: custom_field.value.value ?? "",
  };

  // Clear
  custom_field.value = {
    name: "",
    field_type: "Normal",
    value: "",
  };

  emit("add", { new: clone });
}
</script>

<template>
  <v-form @submit.prevent>
    <v-row class="align-center">
      <v-col cols="2">
        <v-select
          v-if="props.create"
          label="Type"
          :items="['Normal', 'Link']"
          variant="outlined"
          v-model="custom_field.field_type"
          :rules="[(v: any) => !!v || 'Required']"
          required
          :readonly="!props.create"
        ></v-select>
        <v-text-field
          v-else
          label="Type"
          variant="outlined"
          v-model.trim="custom_field.field_type"
          required
          readonly
        ></v-text-field>
      </v-col>
      <v-col>
        <v-text-field
          label="Name"
          variant="outlined"
          v-model.trim="custom_field.name"
          placeholder=""
          :rules="[(v: any) => !!v || 'Required']"
          required
          :readonly="!props.create"
        ></v-text-field>
      </v-col>
      <v-col>
        <v-text-field
          label="Value"
          variant="outlined"
          v-model.trim="custom_field.value"
          placeholder=""
          :readonly="!props.create"
          :dirty="!props.create"
        ></v-text-field>
      </v-col>
      <v-col cols="auto" class="mb-6">
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
      </v-col>
    </v-row>
  </v-form>
</template>
