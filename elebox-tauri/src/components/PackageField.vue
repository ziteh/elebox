<script setup lang="ts">
import { onMounted, ref, reactive } from "vue";
import { DbPackage as Db } from "../db_cmd_package";

const props = defineProps<{ origin_name?: string }>();
const pkg = ref<Db.Package>({ name: "", pkg_type: "", alias: "" });
let pkgs = reactive<Db.Package[]>([]);

async function add() {
  if (pkg.value === undefined) {
    console.warn("undefined");
    return;
  }

  await Db.add(pkg.value);
  await list();
}

async function update() {
  if (props.origin_name === undefined || pkg.value === undefined) {
    return;
  }

  await Db.update(props.origin_name, pkg.value);
  await list();
}

async function list() {
  const data = await Db.list();
  Object.assign(pkgs, data);
  console.debug(`Get packages ${pkgs.length}`);
}

async function get(name: string) {
  const data = await Db.get(name);
  pkg.value = data as Db.Package;
}

onMounted(() => {
  if (props.origin_name !== undefined) {
    get(props.origin_name);
    pkg.value.pkg_type = pkg.value.pkg_type.toUpperCase(); // TODO no effect
  }

  list();
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
          v-model="pkg.pkg_type"
          :rules="[(v: any) => !!v || 'Required']"
          required
        ></v-select>
        <v-text-field
          label="Name"
          variant="outlined"
          v-model="pkg.name"
          placeholder="SOT-23"
          :rules="[(v: any) => !!v || 'Required']"
          required
        ></v-text-field>
        <v-text-field
          label="Alias"
          variant="outlined"
          v-model="pkg.alias"
          placeholder=""
        ></v-text-field>

        <v-btn v-if="props.origin_name === undefined" @click="add">Add</v-btn>
        <v-btn v-else @click="update">Update</v-btn>
      </v-row>
    </v-container>
  </v-form>
</template>
