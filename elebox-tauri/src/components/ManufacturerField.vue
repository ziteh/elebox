<script setup lang="ts">
import { onMounted, ref } from "vue";
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
  <v-form @submit.prevent>
    <v-row class="align-center pb-2">
      <v-col>
        <v-text-field
          label="Name"
          variant="outlined"
          v-model="mfr.name"
          placeholder="Texas Instruments"
          :rules="[(v: any) => !!v || 'Required']"
          required
        ></v-text-field>
      </v-col>
      <v-col>
        <v-text-field
          label="Alias"
          variant="outlined"
          v-model="mfr.alias"
          placeholder="TI"
        ></v-text-field>
      </v-col>
      <v-col>
        <v-text-field
          label="Url"
          variant="outlined"
          v-model="mfr.url"
          placeholder="https://"
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
</template>
