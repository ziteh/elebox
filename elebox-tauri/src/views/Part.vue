<script setup lang="ts">
import { onMounted, ref, reactive } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
// import { VNumberInput } from 'vuetify/labs/VNumberInput'

interface Categories {
  [index: number]: {
    name: string;
    parent: string;
  }
}

interface Package {
  [index: number]: {
    name: string;
    type: string;
    alias: string;
  }
}

interface Manufacturer {
  [index: number]: {
    name: string;
    alias: string;
  }
}

const favorite = ref()
const partName = ref();
const partQty = ref();
const category = ref();
let categories = reactive<Categories>({});
let packages = reactive<Package>({});
let manufacturers = reactive<Manufacturer>({});

const pkg = ref();
const mfr = ref();
const cost = ref();
const location = ref();
const alias = ref();
const description = ref();

async function newPart() {
  await invoke("part_new", { name: partName.value, qty: parseInt(partQty.value), ptype: category.value });
}

async function getCategories() {
  const cs = await invoke("get_categories", {});
  Object.assign(categories, cs);
  console.debug(`get categories: ${categories}`);
}

onMounted(getCategories);
</script>

<template>
  <v-form fast-fail @submit.prevent>
    <v-container>
      <v-row class="mb-3 ga-8" align="center">
        <v-btn type="submit" @click="newPart">ADD</v-btn>
        <v-switch true-icon="mdi-star" v-model="favorite" color="primary" label="Favorite" hide-details></v-switch>
      </v-row>
      <v-row class="ga-8">
        <v-text-field label="Name" variant="outlined" v-model="partName" placeholder="RP2040"
          :rules="[(v: any) => !!v || 'Required']" required></v-text-field>
        <v-text-field label="Quantity" variant="outlined" v-model="partQty" placeholder="15"
          :rules="[(v: any) => !!v || 'Required']" required type="number" min="0"></v-text-field>
        <v-select label="Category" :items="Object.values(categories).map(cat => cat.name)" variant="outlined"
          v-model="category" :rules="[(v: any) => !!v || 'Required']" required></v-select>
      </v-row>

      <v-divider class="ma-8"></v-divider>

      <v-row class="ga-8">
        <v-select label="Package" :items="Object.values(packages).map(pck => pck.name)" variant="outlined"
          v-model="pkg"></v-select>
        <v-select label="Manufacturer" :items="Object.values(manufacturers).map(mfr => mfr.name)" variant="outlined"
          v-model="mfr"></v-select>
        <v-text-field label="Cost" variant="outlined" v-model="cost"></v-text-field>
        <v-text-field label="Location" variant="outlined" v-model="location" placeholder="Box #1"></v-text-field>
      </v-row>
      <v-row class="ga-8">
        <v-text-field label="Alias" variant="outlined" v-model="alias" placeholder=""></v-text-field>
        <v-text-field label="Mfr #" variant="outlined" v-model="pkg" placeholder="SC0914(7)"
          title="Manufacturer #"></v-text-field>
        <v-text-field label="Mouser #" variant="outlined" v-model="pkg" placeholder="358-SC09147"></v-text-field>
        <v-text-field label="Digi-Key #" variant="outlined" v-model="pkg"
          placeholder="2648-SC0914(7)CT-ND"></v-text-field>
      </v-row>
      <v-row class="ga-8">
        <v-text-field label="Product Url" variant="outlined" v-model="pkg" placeholder="https://"></v-text-field>
        <v-text-field label="Datasheet" variant="outlined" v-model="pkg" placeholder="https://"></v-text-field>
        <v-text-field label="Image" variant="outlined" v-model="pkg" placeholder=""></v-text-field>
      </v-row>
      <v-row class="ga-8">
        <v-textarea label="Description" variant="outlined" v-model="description"
          placeholder="Write something ..."></v-textarea>
      </v-row>
      <v-row class="ga-8">
        <v-textarea label="Suppliers" variant="outlined" v-model="pkg" placeholder=""></v-textarea>
      </v-row>
    </v-container>
  </v-form>
</template>
