<script setup lang="ts">
import { onMounted, ref } from "vue";
import { DbPackage as Db } from "../db_cmd_package";
import { PkgType } from "../interface"; // TODO to db_cmd_package

const props = defineProps<{ origin_name?: string }>();
const pkg = ref<Db.Package>({ name: "", pkg_type: PkgType.Smt, alias: "" });
const pkg_type_input = ref<string>("SMT");

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
  await Db.add(pkg.value);
}

async function update() {
  if (props.origin_name === undefined || pkg.value === undefined) {
    return;
  }

  pkg.value.pkg_type = setPkgType(pkg_type_input.value);
  await Db.update(props.origin_name, pkg.value);
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
});
</script>

<template>
  <v-form>
    <v-container>
      <v-row class="ga-8">
        <v-select
          label="Type"
          :items="['SMT', 'THT', 'Others']"
          variant="outlined"
          v-model="pkg_type_input"
          :rules="[(v: any) => !!v || 'Required']"
          required
        ></v-select>
        <v-text-field
          label="Name"
          variant="outlined"
          v-model.trim="pkg.name"
          placeholder="SOT-23"
          :rules="[(v: any) => !!v || 'Required']"
          required
        ></v-text-field>
        <v-text-field
          label="Alias"
          variant="outlined"
          v-model.trim="pkg.alias"
          placeholder=""
        ></v-text-field>

        <v-btn v-if="props.origin_name === undefined" @click="add">Add</v-btn>
        <v-btn v-else @click="update">Update</v-btn>
      </v-row>
    </v-container>
  </v-form>
</template>
