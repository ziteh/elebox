<script setup lang="ts">
import { onMounted, ref, reactive } from "vue";
import { DbPackage as Db } from "../utils/db_cmd_package";
import { PkgType } from "../types/package"; // TODO to db_cmd_package

const props = defineProps<{ origin_name?: string }>();
const pkg = ref<Db.Package>({ name: "", pkg_type: PkgType.Smt, alias: "" });
const pkg_type_input = ref<string>("SMT");
let pkgs = reactive<Db.Package[]>([]);

const snackbar = ref(false);
const snackbar_msg = ref("");
const rules = ref({
  required: (v: any) => !!v || "Required",
  duplicate: (v: any) =>
    !pkgs.some((pkg) => pkg.name === v) || "Already exists",
});

function setPkgType(input: string): PkgType {
  // --noImplicitAny
  // pkg.value.pkg_type = PkgType[pkg_type.value as keyof typeof PkgType];

  if (input === "SMT") {
    return PkgType.Smt;
  } else if (input === "THT") {
    return PkgType.Tht;
  }

  return PkgType.Others;
}

async function add() {
  if (pkg.value === undefined) {
    console.warn("undefined");
    return;
  }

  pkg.value.pkg_type = setPkgType(pkg_type_input.value);
  await Db.add(pkg.value)
    .then(() => {
      snackbar.value = true;
      snackbar_msg.value = "Success";
    })
    .catch((e) => {
      snackbar.value = true;
      snackbar_msg.value = e;
    });
}

async function update() {
  if (props.origin_name === undefined || pkg.value === undefined) {
    return;
  }

  pkg.value.pkg_type = setPkgType(pkg_type_input.value);
  await Db.update(props.origin_name, pkg.value);
}

async function list() {
  const data = await Db.list();
  Object.assign(pkgs, data);
}

async function get(name: string) {
  const data = await Db.get(name);
  pkg.value = data as Db.Package;

  if (pkg.value.pkg_type == PkgType.Smt) {
    pkg_type_input.value = "SMT";
  } else if (pkg.value.pkg_type == PkgType.Tht) {
    pkg_type_input.value = "THT";
  } else {
    pkg_type_input.value = "Others";
  }
}

onMounted(() => {
  if (props.origin_name !== undefined) {
    get(props.origin_name);
  }

  list();
});
</script>

<template>
  <v-form @submit.prevent>
    <v-row class="align-center pb-2">
      <v-col>
        <v-select
          label="Type"
          :items="['SMT', 'THT', 'Others']"
          variant="outlined"
          v-model="pkg_type_input"
          :rules="[(v: any) => !!v || 'Required']"
          required
        ></v-select>
      </v-col>
      <v-col>
        <v-text-field
          label="Name"
          variant="outlined"
          v-model.trim="pkg.name"
          placeholder="SOT-23"
          :rules="[rules.required, rules.duplicate]"
          required
        ></v-text-field>
      </v-col>
      <v-col>
        <v-text-field
          label="Alias"
          variant="outlined"
          v-model.trim="pkg.alias"
          placeholder=""
        ></v-text-field>
      </v-col>
      <v-col cols="auto" class="mb-6">
        <v-btn
          v-if="props.origin_name === undefined"
          @click="add"
          type="submit"
          text="Add"
        ></v-btn>
        <v-btn v-else @click="update" type="submit" text="Update"></v-btn>
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
