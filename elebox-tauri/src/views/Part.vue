<script setup lang="ts">
import { onMounted, ref, reactive } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
// import { VNumberInput } from 'vuetify/labs/VNumberInput'

interface Categories {
  [index: number]: {
    name: string;
    parent: string;
  };
}

interface Package {
  [index: number]: {
    pname: string;
    type: string;
    alias: string;
  };
}

interface Manufacturer {
  [index: number]: {
    name: string;
    alias: string;
    url: string;
  };
}
let categories = reactive<Categories>({});
let packages = reactive<Package>({});
let manufacturers = reactive<Manufacturer>({});

const favorite = ref();

const name = ref();
const qty = ref();
const category = ref();
const pkg = ref();
const mfr = ref();
const cost = ref();
const location = ref();
const alias = ref();
const description = ref();
const mfr_no = ref();
const mouser_no = ref();
const digikey_no = ref();
const datasheet_link = ref();
const product_link = ref();
const image_link = ref();
const suppliers = ref();

async function newPart() {
  console.log(mfr.value);
  console.log(pkg.value);

  if (cost.value === undefined || cost.value === null || cost.value === "") {
    cost.value = "-9999";
  }

  await invoke("new_part", {
    name: name.value,
    qty: parseInt(qty.value),
    category: category.value,
    package: pkg.value ?? "",
    mfr: mfr.value ?? "",
    alias: alias.value ?? "",
    description: description.value ?? "",
    cost: parseFloat(cost.value),
    location: location.value ?? "",
    mfrNo: mfr_no.value ?? "",
    mouserNo: mouser_no.value ?? "",
    digikeyNo: digikey_no.value ?? "",
    datasheetLink: datasheet_link.value ?? "",
    productLink: product_link.value ?? "",
    imageLink: image_link.value ?? "",
    suppliers: suppliers.value ?? "",
  });
}

async function getCategories() {
  const cs = await invoke("get_categories", {});
  Object.assign(categories, cs);
  console.debug(`get categories: ${categories}`);
}

async function getMfrs() {
  const cs = await invoke("get_mfrs", {});
  Object.assign(manufacturers, cs);
}

async function getPackages() {
  const cs = await invoke("get_packages", {});
  Object.assign(packages, cs);
}

onMounted(() => {
  getCategories();
  getMfrs();
  getPackages();
});
</script>

<template>
  <v-form fast-fail @submit.prevent>
    <v-container>
      <v-row class="mb-3 ga-8" align="center">
        <v-btn type="submit" @click="newPart">ADD</v-btn>
        <v-switch
          true-icon="mdi-star"
          v-model="favorite"
          color="primary"
          label="Favorite"
          hide-details
        ></v-switch>
      </v-row>
      <v-row class="ga-8">
        <v-text-field
          label="Name"
          variant="outlined"
          v-model="name"
          placeholder="RP2040"
          :rules="[(v: any) => !!v || 'Required']"
          required
        ></v-text-field>
        <v-text-field
          label="Quantity"
          variant="outlined"
          v-model="qty"
          placeholder="15"
          :rules="[(v: any) => !!v || 'Required']"
          required
          type="number"
          min="0"
        ></v-text-field>
        <v-select
          label="Category"
          :items="Object.values(categories).map((cat) => cat.name)"
          variant="outlined"
          v-model="category"
          :rules="[(v: any) => !!v || 'Required']"
          required
        ></v-select>
      </v-row>

      <v-divider class="ma-8"></v-divider>

      <v-row class="ga-8">
        <v-select
          label="Package"
          :items="Object.values(packages).map((pck) => pck.name)"
          variant="outlined"
          v-model="pkg"
        ></v-select>
        <v-select
          label="Manufacturer"
          :items="Object.values(manufacturers).map((mfr) => mfr.name)"
          variant="outlined"
          v-model="mfr"
        ></v-select>
        <v-text-field
          label="Cost"
          variant="outlined"
          v-model="cost"
          type="number"
          min="0"
        ></v-text-field>
        <v-text-field
          label="Location"
          variant="outlined"
          v-model="location"
          placeholder="Box #1"
        ></v-text-field>
      </v-row>
      <v-row class="ga-8">
        <v-text-field
          label="Alias"
          variant="outlined"
          v-model="alias"
          placeholder=""
        ></v-text-field>
        <v-text-field
          label="Mfr #"
          variant="outlined"
          v-model="mfr_no"
          placeholder="SC0914(7)"
          title="Manufacturer part number"
        ></v-text-field>
        <v-text-field
          label="Mouser #"
          variant="outlined"
          v-model="mouser_no"
          placeholder="358-SC09147"
        ></v-text-field>
        <v-text-field
          label="Digi-Key #"
          variant="outlined"
          v-model="digikey_no"
          placeholder="2648-SC0914(7)CT-ND"
        ></v-text-field>
      </v-row>
      <v-row class="ga-8">
        <v-text-field
          label="Product"
          variant="outlined"
          v-model="product_link"
          placeholder="URL, full path or filename"
        ></v-text-field>
        <v-text-field
          label="Datasheet"
          variant="outlined"
          v-model="datasheet_link"
          placeholder="URL, full path or filename"
        ></v-text-field>
        <v-text-field
          label="Image"
          variant="outlined"
          v-model="image_link"
          placeholder="URL, full path or filename"
        ></v-text-field>
      </v-row>
      <v-row class="ga-8">
        <v-textarea
          label="Description"
          variant="outlined"
          v-model="description"
          placeholder="Write something ..."
        ></v-textarea>
      </v-row>
      <v-row class="ga-8">
        <v-textarea
          label="Suppliers"
          variant="outlined"
          v-model="suppliers"
          placeholder=""
        ></v-textarea>
      </v-row>
    </v-container>
  </v-form>
</template>
