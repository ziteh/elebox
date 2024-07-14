<script setup lang="ts">
import { onMounted, ref, reactive } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import {
  Categories,
  Packages,
  Manufacturers,
  CustomField,
  Supplier,
  PartData,
} from "../interface";
import PartCustomField from "../components/PartCustomField.vue";
import PartSupplier from "../components/PartSupplier.vue";
import { useRoute } from "vue-router";

let categories = reactive<Categories>({});
let packages = reactive<Packages>({});
let manufacturers = reactive<Manufacturers>({});
let custom_fields = reactive<CustomField[]>([]);
let suppliers = reactive<Supplier[]>([]);

const new_cf_type = ref();
const new_cf_name = ref();
const new_cf_value = ref();

const new_s_name = ref();
const new_s_link = ref();
const new_s_price = ref();
const new_s_note = ref();

const favorite = ref();

const name = ref();
const qty = ref();
const category = ref();
const pkg = ref();
const pkg_detail = ref();
const mfr = ref();
const location = ref();
const alias = ref();
const description = ref();
const mfr_no = ref();
const datasheet_link = ref();
const product_link = ref();
const image_link = ref();

const ori_name = ref();

async function newPart() {
  // console.log(mfr.value);
  // console.log(pkg.value);

  await invoke("new_part", {
    name: name.value,
    qty: parseInt(qty.value),
    category: category.value,
    package: pkg.value ?? "",
    packageDetail: pkg_detail.value ?? "",
    mfr: mfr.value ?? "",
    alias: alias.value ?? "",
    description: description.value ?? "",
    location: location.value ?? "",
    mfrNo: mfr_no.value ?? "",
    datasheetLink: datasheet_link.value ?? "",
    productLink: product_link.value ?? "",
    imageLink: image_link.value ?? "",
    customFields: custom_fields,
    suppliers: suppliers,
  });
}
async function updatePart() {
  await invoke("update_part", {
    originName: ori_name.value,
    name: name.value,
    qty: parseInt(qty.value),
    category: category.value,
    package: pkg.value ?? "",
    packageDetail: pkg_detail.value ?? "",
    mfr: mfr.value ?? "",
    alias: alias.value ?? "",
    description: description.value ?? "",
    location: location.value ?? "",
    mfrNo: mfr_no.value ?? "",
    datasheetLink: datasheet_link.value ?? "",
    productLink: product_link.value ?? "",
    imageLink: image_link.value ?? "",
    customFields: custom_fields,
    suppliers: suppliers,
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

async function getPart(ori_name: string) {
  const cs = await invoke("get_part", { part: ori_name });
  const ori_part = cs as PartData;

  name.value = ori_part.name;
  qty.value = ori_part.quantity;
  category.value = ori_part.category;
  pkg.value = ori_part.package;
  pkg_detail.value = ori_part.package_detail;
  mfr.value = ori_part.mfr;
  mfr_no.value = ori_part.mfr_no;
  alias.value = ori_part.alias;
  product_link.value = ori_part.product_link;
  datasheet_link.value = ori_part.datasheet_link;
  image_link.value = ori_part.image_link;
  description.value = ori_part.description;
  Object.assign(suppliers, ori_part.suppliers);
  Object.assign(custom_fields, ori_part.custom_fields);
}

function handleCustomFieldUpdate(data: {
  field_type: string;
  name: string;
  value: string;
}) {
  new_cf_type.value = data.field_type;
  new_cf_name.value = data.name;
  new_cf_value.value = data.value;
}

function handleCustomFieldDel(data: { name: string }) {
  // Find by name
  const index = custom_fields.findIndex((f) => f.name === data.name);
  if (index !== -1) {
    custom_fields.splice(index, 1); // Remove
  }
}

function handleCustomFieldAdd() {
  const cf: CustomField = {
    field_type: new_cf_type.value,
    name: new_cf_name.value,
    value: new_cf_value.value || "",
  };
  custom_fields.push(cf);

  new_cf_type.value = "";
  new_cf_name.value = "";
  new_cf_value.value = "";
}

function handleSupplierUpdate(data: {
  name: string;
  link: string;
  price: number;
  note: string;
}) {
  new_s_name.value = data.name;
  new_s_link.value = data.link;
  new_s_price.value = data.price;
  new_s_note.value = data.note;
}

function handleSupplierDel(data: { name: string }) {
  // Find by name
  const index = suppliers.findIndex((s) => s.name === data.name);
  if (index !== -1) {
    suppliers.splice(index, 1); // Remove
  }
}

function handleSupplierAdd() {
  const sup: Supplier = {
    name: new_s_name.value,
    link: new_s_link.value || "",
    note: new_s_note.value || "",
    price: new_s_price?.value ? parseFloat(new_s_price.value) : undefined,
  };
  suppliers.push(sup);

  new_s_name.value = "";
  new_s_link.value = "";
  new_s_price.value = 0.0;
  new_s_note.value = "";
}

const route = useRoute();

onMounted(() => {
  ori_name.value = route.params.ori_name || "";
  if (ori_name.value !== "") {
    getPart(ori_name.value);
  }
  getCategories();
  getMfrs();
  getPackages();
});
</script>

<template>
  <v-form fast-fail @submit.prevent>
    <v-container>
      <v-row class="mb-3 ga-8" align="center">
        <v-btn v-if="ori_name == ''" type="submit" @click="newPart">ADD</v-btn>
        <v-btn v-else @click="updatePart">Update</v-btn>
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

      <v-card title="Custom Fields" variant="flat">
        <v-container>
          <v-row class="ga-8" v-for="cf in custom_fields">
            <PartCustomField
              :field_type="cf.field_type"
              :name="cf.name"
              :value="cf.value"
              :create="false"
              @del="handleCustomFieldDel"
            />
          </v-row>
          <v-row class="ga-8">
            <PartCustomField
              :field_type="new_cf_type"
              :name="new_cf_name"
              :value="new_cf_type"
              :create="true"
              @update="handleCustomFieldUpdate"
              @add="handleCustomFieldAdd"
            />
          </v-row>
        </v-container>
      </v-card>

      <v-card title="Suppliers" variant="flat">
        <v-container>
          <v-row class="ga-8" v-for="s in suppliers">
            <PartSupplier
              :name="s.name"
              :link="s.link"
              :price="s.price"
              :note="s.note"
              :create="false"
              @del="handleSupplierDel"
            />
          </v-row>
          <v-row class="ga-8">
            <PartSupplier
              :name="new_s_name"
              :link="new_s_link"
              :price="new_s_price"
              :note="new_s_note"
              :create="true"
              @update="handleSupplierUpdate"
              @add="handleSupplierAdd"
            />
          </v-row>
        </v-container>
      </v-card>
    </v-container>
  </v-form>
</template>
