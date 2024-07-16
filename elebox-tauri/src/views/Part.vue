<script setup lang="ts">
import { onMounted, ref, reactive } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { CustomField, Supplier } from "../interface";
import PartCustomField from "../components/PartCustomField.vue";
import PartSupplier from "../components/PartSupplier.vue";
import { useRoute } from "vue-router";
import { DbPart } from "../db_cmd_part";
import { DbCategory } from "../db_cmd_category";
import { DbManufacturer as DbMfr } from "../db_cmd_manufacturer";
import { DbPackage } from "../db_cmd_package";

const origin_name = ref<string>();
const part = ref<DbPart.Part>({
  name: "",
  quantity: 0,
  category: "",
  custom_fields: [],
  suppliers: [],
});

const favorite = ref();

const new_custom_field = ref<CustomField>({
  name: "",
  field_type: "",
  value: "",
});

const new_supplier = ref<Supplier>({
  name: "",
  link: "",
  price: undefined,
  note: "",
});

let categories = reactive<DbCategory.Category[]>([]);
let packages = reactive<DbPackage.Package[]>([]);
let mfrs = reactive<DbMfr.Manufacturer[]>([]);
let custom_fields = reactive<CustomField[]>([]);
let suppliers = reactive<Supplier[]>([]);

const new_cf_type = ref();
const new_cf_name = ref();
const new_cf_value = ref();

const new_s_name = ref();
const new_s_link = ref();
const new_s_price = ref();
const new_s_note = ref();

async function newPart() {
  if (
    part.value.name === "" ||
    part.value.category === "" ||
    part.value.quantity < 0
  ) {
    return;
  }

  part.value.quantity = Math.trunc(part.value.quantity);
  await DbPart.add(part.value);
}

async function updatePart() {
  if (origin_name.value === undefined) {
    return;
  }

  if (
    part.value.name === "" ||
    part.value.category === "" ||
    part.value.quantity < 0
  ) {
    return;
  }

  await DbPart.update(origin_name.value, part.value);
}

async function getCategories() {
  const data = await DbCategory.list();
  Object.assign(categories, data);
}

async function getMfrs() {
  const date = await DbMfr.list();
  Object.assign(mfrs, date);
}

async function getPackages() {
  const data = await DbPackage.list();
  Object.assign(packages, data);
}

async function getPart(name: string) {
  const data = await DbPart.get(name);
  part.value = data as DbPart.Part;
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
  origin_name.value = route.params.ori_name || "";
  if (origin_name.value !== "") {
    getPart(origin_name.value);
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
        <v-btn v-if="origin_name == ''" type="submit" @click="newPart"
          >ADD</v-btn
        >
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
          v-model="part.name"
          placeholder="RP2040"
          :rules="[(v: any) => !!v || 'Required']"
          required
        ></v-text-field>
        <v-text-field
          label="Quantity"
          variant="outlined"
          v-model.number="part.quantity"
          placeholder="15"
          :rules="[(v: any) => !!v || 'Required']"
          required
          type="number"
          min="0"
        ></v-text-field>
        <v-autocomplete
          label="Category"
          variant="outlined"
          v-model="part.category"
          :items="Object.values(categories).map((cat) => cat.name)"
          :rules="[(v: any) => !!v || 'Required']"
        ></v-autocomplete>
      </v-row>

      <v-divider class="ma-8"></v-divider>

      <v-row class="ga-8">
        <v-autocomplete
          label="Package"
          variant="outlined"
          v-model="part.package"
          :items="Object.values(packages).map((pck) => pck.name)"
        ></v-autocomplete>
        <v-text-field
          label="Package Detail"
          variant="outlined"
          v-model="part.package_detail"
          placeholder="7x7mm P0.4mm 1EP3.2x32.mm"
        ></v-text-field>
        <v-autocomplete
          label="Manufacturer"
          variant="outlined"
          v-model="part.mfr"
          :items="Object.values(mfrs).map((mfr) => mfr.name)"
        ></v-autocomplete>
        <v-text-field
          label="Location"
          variant="outlined"
          v-model="part.location"
          placeholder="Box #1"
        ></v-text-field>
      </v-row>
      <v-row class="ga-8">
        <v-text-field
          label="Alias"
          variant="outlined"
          v-model="part.alias"
          placeholder=""
        ></v-text-field>
        <v-text-field
          label="Mfr #"
          variant="outlined"
          v-model="part.mfr_no"
          placeholder="SC0914(7)"
          title="Manufacturer part number"
        ></v-text-field>
      </v-row>
      <v-row class="ga-8">
        <v-text-field
          label="Product"
          variant="outlined"
          v-model="part.product_link"
          placeholder="URL, full path or filename"
        ></v-text-field>
        <v-text-field
          label="Datasheet"
          variant="outlined"
          v-model="part.datasheet_link"
          placeholder="URL, full path or filename"
        ></v-text-field>
        <v-text-field
          label="Image"
          variant="outlined"
          v-model="part.image_link"
          placeholder="URL, full path or filename"
        ></v-text-field>
      </v-row>
      <v-row class="ga-8">
        <v-textarea
          label="Description"
          variant="outlined"
          v-model="part.description"
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
