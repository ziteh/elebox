<script setup lang="ts">
import { onMounted, ref, reactive } from "vue";
import { DbManufacturer as Db } from "../db_cmd_manufacturer";

const props = defineProps<{ origin_name?: string }>();
const mfr = ref<Db.Manufacturer>({ name: "", url: "", alias: "" });

async function add() {
  if (mfr.value === undefined) {
    console.warn("undefined");
    return;
  }

  await Db.add(mfr.value);
}

async function update() {
  if (props.origin_name === undefined || mfr.value === undefined) {
    return;
  }

  await Db.update(props.origin_name, mfr.value);
}

async function get(name: string) {
  const data = await Db.get(name);
  mfr.value = data as Db.Manufacturer;
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
        <v-text-field
          label="Name"
          variant="outlined"
          v-model="mfr.name"
          placeholder="Texas Instruments"
          :rules="[(v: any) => !!v || 'Required']"
          required
        ></v-text-field>
        <v-text-field
          label="Alias"
          variant="outlined"
          v-model="mfr.alias"
          placeholder="TI"
        ></v-text-field>
        <v-text-field
          label="Url"
          variant="outlined"
          v-model="mfr.url"
          placeholder="https://"
        ></v-text-field>

        <v-btn v-if="props.origin_name === undefined" @click="add">Add</v-btn>
        <v-btn v-else @click="update">Update</v-btn>
      </v-row>
    </v-container>
  </v-form>
</template>
