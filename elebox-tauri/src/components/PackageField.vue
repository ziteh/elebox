<script setup lang="ts">
import { onMounted, ref, reactive } from "vue";
import { DbPackage as Db } from "@/utils/db_cmd_package";
import { PkgType } from "@/types/package"; // TODO to db_cmd_package

const props = defineProps<{
  ori_name?: string; // If ori_name undefined: create mode, otherwise edit mode
}>();

const current = ref<Db.Package>({ name: "", pkg_type: PkgType.Smt, alias: "" });
const existing = reactive<string[]>([]);
const pkg_type_input = ref<string>("SMT");

const snackbar = ref(false);
const snackbar_msg = ref("");

const rules = {
  required: (val: any) => !!val || "Required",
  duplicate: (val: any) => !existing.some((i) => i === val) || "Already exists",
};

function toPkgType(input: string): PkgType {
  if (input === "SMT") {
    return PkgType.Smt;
  } else if (input === "THT") {
    return PkgType.Tht;
  }

  return PkgType.Others;
}

async function createNew() {
  if (current.value == undefined) {
    return;
  }

  current.value.pkg_type = toPkgType(pkg_type_input.value);

  await Db.add(current.value)
    .then(() => {
      snackbar.value = true;
      snackbar_msg.value = "Success";
      fetchExisting();
    })
    .catch((err) => {
      snackbar.value = true;
      snackbar_msg.value = err;
    });
}

async function updateOriginal() {
  if (props.ori_name == undefined || current.value == undefined) {
    return;
  }

  current.value.pkg_type = toPkgType(pkg_type_input.value);
  await Db.update(props.ori_name, current.value);
}

async function fetchExisting() {
  const data = await Db.list();
  Object.assign(
    existing,
    data.map((i) => i.name)
  );

  console.debug(`Get packages: ${existing.length}`);

  // For rules duplicate
  if (props.ori_name) {
    const self_index = existing.findIndex((i) => i === props.ori_name);
    if (self_index !== -1) {
      existing.splice(self_index, 1); // Remove self from existing categories
    }
  }
}

async function fetchCurrent() {
  if (props.ori_name) {
    const data = await Db.get(props.ori_name);
    current.value = data as Db.Package;
  }

  switch (current.value.pkg_type) {
    case PkgType.Smt:
      pkg_type_input.value = "SMT";
      break;

    case PkgType.Tht:
      pkg_type_input.value = "THT";
      break;

    default:
      pkg_type_input.value = "Others";
      break;
  }
}

onMounted(() => {
  fetchCurrent();
  fetchExisting();
});
</script>

<template>
  <v-form @submit.prevent>
    <v-row class="align-center pb-2">
      <v-col>
        <v-select
          label="Type"
          variant="outlined"
          :items="['SMT', 'THT', 'Others']"
          v-model="pkg_type_input"
          :rules="[rules.required]"
          required
        ></v-select>
      </v-col>
      <v-col>
        <v-text-field
          label="Name"
          variant="outlined"
          v-model.trim="current.name"
          placeholder="SOT-23"
          :rules="[rules.required, rules.duplicate]"
          required
        ></v-text-field>
      </v-col>
      <v-col>
        <v-text-field
          label="Alias"
          variant="outlined"
          v-model.trim="current.alias"
        ></v-text-field>
      </v-col>
      <v-col cols="auto" class="mb-6">
        <v-btn
          v-if="props.ori_name == undefined"
          type="submit"
          @click="createNew"
          >Add</v-btn
        >
        <v-btn v-else type="submit" @click="updateOriginal">Update</v-btn>
      </v-col>
    </v-row>
  </v-form>

  <v-snackbar v-model="snackbar">
    {{ snackbar_msg }}
    <template v-slot:actions>
      <v-btn color="pink" variant="text" @click="snackbar = false">
        Close
      </v-btn>
    </template>
  </v-snackbar>
</template>
