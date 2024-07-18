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

const origin_name = ref<string>("");
const part = ref<DbPart.Part>({
  name: "",
  quantity: 0,
  category: "",
  custom_fields: [],
  suppliers: [],
});

const qty_input = ref<number | undefined>(undefined);
const favorite = ref();
const alert = ref(false);

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

async function newPart() {
  if (origin_name.value === undefined || qty_input.value === undefined) {
    return;
  }

  part.value.quantity = qty_input.value;

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
  if (origin_name.value === undefined || qty_input.value === undefined) {
    return;
  }

  part.value.quantity = qty_input.value;

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
  qty_input.value = part.value.quantity;
  Object.assign(custom_fields, part.value.custom_fields);
  Object.assign(suppliers, part.value.suppliers);
}

function handleCustomFieldUpdate(data: { new: CustomField }) {
  new_custom_field.value = data.new;
}

function handleCustomFieldDel(data: { name: string }) {
  // Find by name
  const index = custom_fields.findIndex((f) => f.name === data.name);
  if (index !== -1) {
    custom_fields.splice(index, 1); // Remove
  }
}

function handleCustomFieldAdd(data: { new: CustomField }) {
  new_custom_field.value = {
    field_type: "",
    name: "",
    value: "",
  };
  custom_fields.push(data.new);
}

function handleSupplierUpdate(data: { new: Supplier }) {
  new_supplier.value = data.new;
}

function handleSupplierDel(data: { name: string }) {
  // Find by name
  const index = suppliers.findIndex((s) => s.name === data.name);
  if (index !== -1) {
    suppliers.splice(index, 1); // Remove
  }
}

function handleSupplierAdd(data: { new: Supplier }) {
  suppliers.push(data.new);
  new_supplier.value = {
    name: "",
    link: "",
    note: "",
    price: undefined,
  };
}

const route = useRoute();

onMounted(() => {
  if (
    route.params.origin_name !== undefined &&
    route.params.origin_name !== ""
  ) {
    origin_name.value = route.params.origin_name; // FIXME
    getPart(origin_name.value);
  }

  getCategories();
  getMfrs();
  getPackages();
});
</script>

<template>
  <v-form @submit.prevent>
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
          v-model.number="qty_input"
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

      <v-card title="Custom Fields" variant="flat" class="mb-2">
        <v-container>
          <v-row class="ma-2" v-for="cf in custom_fields">
            <PartCustomField
              :field_type="cf.field_type"
              :name="cf.name"
              :value="cf.value"
              :create="false"
              @del="handleCustomFieldDel"
            />
          </v-row>
          <v-row class="ma-2">
            <PartCustomField
              :field_type="new_custom_field.field_type"
              :name="new_custom_field.name"
              :value="new_custom_field.value"
              :create="true"
              @update="handleCustomFieldUpdate"
              @add="handleCustomFieldAdd"
            />
          </v-row>
        </v-container>
      </v-card>

      <v-card title="Suppliers" variant="flat">
        <v-container>
          <v-row class="ma-2" v-for="s in suppliers">
            <PartSupplier
              :name="s.name"
              :link="s.link"
              :price="s.price"
              :note="s.note"
              :create="false"
              @del="handleSupplierDel"
            />
          </v-row>
          <v-row class="ma-2">
            <PartSupplier
              :name="new_supplier.name"
              :link="new_supplier.link"
              :price="new_supplier.price"
              :note="new_supplier.note"
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
