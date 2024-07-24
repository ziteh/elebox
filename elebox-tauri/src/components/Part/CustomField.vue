<script setup lang="ts">
import { ref, watch } from "vue";
import { CustomField } from "@/types/part";
import { makeRules } from "@/utils/input_rules";

const props = defineProps<{
  current?: CustomField;
  existing?: CustomField[];
}>();

const custom_field = ref<CustomField>({
  field_type: props.current?.field_type ?? "Normal",
  name: props.current?.name ?? "",
  value: props.current?.value ?? "",
});

const rules = makeRules(props.existing);

const emit = defineEmits(["update", "add", "del"]);

watch([custom_field], ([new_custom_field]) => {
  emit("update", { new: new_custom_field });
});

function emitDel() {
  emit("del", { name: props.current?.name });
}

function emitAdd() {
  // Required value
  if (
    !custom_field.value.field_type ||
    !custom_field.value.name ||
    rules.value.duplicate(custom_field.value.name) !== true
  ) {
    return;
  }

  const clone: CustomField = {
    name: custom_field.value.name,
    field_type: custom_field.value.field_type,
    value: custom_field.value.value ?? "",
  };

  // Clear
  custom_field.value = {
    field_type: "Normal",
    name: "",
    value: "",
  };

  emit("add", { new: clone });
}
</script>

<template>
  <v-form @submit.prevent>
    <v-row class="align-center">
      <v-col cols="2">
        <v-text-field
          v-if="props.existing == undefined"
          label="Type"
          variant="outlined"
          v-model.trim="custom_field.field_type"
          required
          readonly
        >
        </v-text-field>
        <v-select
          v-else
          label="Type"
          :items="['Normal', 'Link']"
          variant="outlined"
          v-model="custom_field.field_type"
          :rules="[rules.required]"
          required
          :readonly="props.existing == undefined"
        ></v-select>
      </v-col>
      <v-col>
        <v-text-field
          label="Name"
          variant="outlined"
          v-model.trim="custom_field.name"
          placeholder=""
          :rules="[rules.required, rules.duplicate]"
          required
          :readonly="props.existing == undefined"
        ></v-text-field>
      </v-col>
      <v-col>
        <v-text-field
          label="Value"
          variant="outlined"
          v-model.trim="custom_field.value"
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
