<script setup lang="ts">
import { onMounted, ref, reactive } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { Manufacturers } from "../interface";

let manufacturers = reactive<Manufacturers>({});

const name = ref("");
const alias = ref("");
const url = ref("");

async function newMfr() {
  await invoke("new_mfr", {
    name: name.value,
    alias: alias.value,
    url: url.value,
  });

  await getMfrs();
}

async function delMfr(name: string) {
  await invoke("del_mfr", { name });
  await getMfrs();
}

async function getMfrs() {
  const cs = await invoke("get_mfrs", {});
  Object.assign(manufacturers, cs);
}

onMounted(getMfrs);
</script>

<template>
  <v-container>
    <v-form fast-fail @submit.prevent>
      <v-container>
        <v-row class="ga-8">
          <v-text-field
            label="Name"
            variant="outlined"
            v-model="name"
            placeholder="Texas Instruments"
            :rules="[(v: any) => !!v || 'Required']"
            required
          ></v-text-field>
          <v-text-field
            label="Alias"
            variant="outlined"
            v-model="alias"
            placeholder="TI"
          ></v-text-field>
          <v-btn type="submit" @click="newMfr">Add</v-btn>
        </v-row>
        <v-row>
          <v-text-field
            label="Url"
            variant="outlined"
            v-model="url"
            placeholder="https://"
          ></v-text-field>
        </v-row>
      </v-container>
    </v-form>
    <v-table>
      <thead>
        <tr>
          <th>Name</th>
          <th>Alias</th>
          <th>Edit</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="(mfr, i) in manufacturers" :key="i">
          <td>{{ mfr.name }}</td>
          <td>{{ mfr.alias }}</td>
          <td>
            <v-btn
              density="comfortable"
              icon="mdi-trash-can-outline"
              @click="delMfr(mfr.name)"
              title="Delete"
            ></v-btn>
          </td>
        </tr>
      </tbody>
    </v-table>
  </v-container>
</template>
