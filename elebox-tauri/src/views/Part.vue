<script setup lang="ts">
import { onMounted, ref, reactive } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { Categories, Packages, Manufacturers, CustomField } from "../interface";
import PartCustomField from "../components/PartCustomField.vue";

// import { VNumberInput } from 'vuetify/labs/VNumberInput'

let categories = reactive<Categories>({});
let packages = reactive<Packages>({});
let manufacturers = reactive<Manufacturers>({});

let custom_fields = reactive<CustomField[]>([
  {
    field_type: "Link",
    name: "AA",
    value: "123",
  },
  {
    field_type: "Normal",
    name: "BB",
    value: "123lasdkfj",
  },
]);

const new_cf_type = ref();
const new_cf_name = ref();
const new_cf_value = ref();

const favorite = ref();

const name = ref();
const qty = ref();
const category = ref();
const pkg = ref();
const pkg_detail = ref();
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
    package_detail: pkg_detail.value ?? "",
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

function handleUpdate(data: {
  field_type: string;
  name: string;
  value: string;
}) {
  new_cf_type.value = data.field_type;
  new_cf_name.value = data.name;
  new_cf_value.value = data.value;
}

function handleDel(data: { name: String }) {
  // Find by name
  const index = custom_fields.findIndex((f) => f.name === data.name);
  if (index !== -1) {
    custom_fields.splice(index, 1); // Remove
  }
}

function handleAdd(data: { field_type: string; name: string; value: string }) {
  custom_fields.push(data);
  new_cf_type.value = "";
  new_cf_name.value = "";
  new_cf_value.value = "";
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
        <v-autocomplete
          label="Category"
          variant="outlined"
          v-model="category"
          :items="Object.values(categories).map((cat) => cat.name)"
          :rules="[(v: any) => !!v || 'Required']"
        ></v-autocomplete>
      </v-row>

      <v-divider class="ma-8"></v-divider>

      <v-row class="ga-8">
        <v-autocomplete
          label="Package"
          variant="outlined"
          v-model="pkg"
          :items="Object.values(packages).map((pck) => pck.name)"
        ></v-autocomplete>
        <v-text-field
          label="Package Detail"
          variant="outlined"
          v-model="pkg_detail"
          placeholder="7x7mm P0.4mm 1EP3.2x32.mm"
        ></v-text-field>
        <v-autocomplete
          label="Manufacturer"
          variant="outlined"
          v-model="mfr"
          :items="Object.values(manufacturers).map((mfr) => mfr.name)"
        ></v-autocomplete>
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
      <v-row class="ga-8" v-for="cf in custom_fields">
        <PartCustomField
          :field_type="cf.field_type"
          :name="cf.name"
          :value="cf.value"
          :create="false"
          @del="handleDel"
        />
      </v-row>
      <v-row class="ga-8">
        <PartCustomField
          :field_type="new_cf_type"
          :name="new_cf_name"
          :value="new_cf_type"
          :create="true"
          @update="handleUpdate"
          @add="handleAdd"
        />
      </v-row>
    </v-container>
  </v-form>
</template>
