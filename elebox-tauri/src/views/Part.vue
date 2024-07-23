<script setup lang="ts">
import { onMounted, ref, reactive } from "vue";
import { useRoute, useRouter } from "vue-router";
import { DbPart } from "../utils/db_cmd_part";
import { DbCategory } from "../utils/db_cmd_category";
import { DbPackage } from "../utils/db_cmd_package";
import { DbManufacturer as DbMfr } from "../utils/db_cmd_manufacturer";
import { CustomField, Supplier } from "../types/part";
import PartCustomField from "../components/PartCustomField.vue";
import PartSupplier from "../components/PartSupplier.vue";

const snackbar = ref(false);
const snackbar_msg = ref("");
const rules = ref({
  required: (v: any) => !!v || "Required",
  duplicate: (v: any) =>
    !parts.some((part) => part.name === v) || "Already exists",
});

const router = useRouter();
const origin_name = ref<string>("");
const part = ref<DbPart.Part>({
  name: "",
  quantity: 0,
  category: "",
  custom_fields: [],
  suppliers: [],
  starred: false,
});

const qty_input = ref<number | undefined>(undefined);

const new_custom_field = ref<CustomField>({
  field_type: "Normal",
  name: "",
  value: "",
});

const new_supplier = ref<Supplier>({
  name: "",
  link: "",
  price: undefined,
  note: "",
});

let parts = reactive<DbPart.Part[]>([]);
let categories = reactive<DbCategory.Category[]>([]);
let packages = reactive<DbPackage.Package[]>([]);
let mfrs = reactive<DbMfr.Manufacturer[]>([]);
let custom_fields = reactive<CustomField[]>([]);
let suppliers = reactive<Supplier[]>([]);

async function newPart() {
  if (origin_name.value === undefined || qty_input.value === undefined) {
    return;
  }

  part.value.quantity = Math.trunc(qty_input.value);

  if (
    part.value.name === "" ||
    part.value.category === "" ||
    part.value.quantity < 0
  ) {
    return;
  }

  if (part.value.mfr === "") {
    part.value.mfr = undefined;
  }
  if (part.value.package === "") {
    part.value.package = undefined;
  }

  part.value.custom_fields = custom_fields;
  part.value.suppliers = suppliers;
  await DbPart.add(part.value)
    .then(() => router.replace("/")) // Back to home
    .catch((err) => {
      snackbar.value = true;
      snackbar_msg.value = err;
    });
}

async function updatePart() {
  if (origin_name.value === undefined || qty_input.value === undefined) {
    return;
  }

  part.value.quantity = Math.trunc(qty_input.value);

  if (
    part.value.name === "" ||
    part.value.category === "" ||
    part.value.quantity < 0
  ) {
    return;
  }

  if (part.value.mfr === "") {
    part.value.mfr = undefined;
  }
  if (part.value.package === "") {
    part.value.package = undefined;
  }

  part.value.custom_fields = custom_fields;
  part.value.suppliers = suppliers;
  await DbPart.update(origin_name.value, part.value)
    .then(() => router.replace("/")) // Back to home
    .catch((err) => {
      snackbar.value = true;
      snackbar_msg.value = err;
    });
}

async function getParts() {
  const data = await DbPart.list();
  Object.assign(parts, data);
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
}

const route = useRoute();

onMounted(() => {
  if (
    route.params.origin_name !== undefined &&
    route.params.origin_name !== ""
  ) {
    if (Array.isArray(route.params.origin_name)) {
      origin_name.value = route.params.origin_name[0];
    } else {
      origin_name.value = route.params.origin_name;
    }

    getPart(origin_name.value);
  }

  getParts();
  getCategories();
  getMfrs();
  getPackages();
});
</script>

<template>
  <v-form @submit.prevent>
    <v-container>
      <v-row align="center">
        <v-col><h1>Part</h1></v-col>
        <v-spacer></v-spacer>
        <v-col cols="auto">
          <v-switch
            true-icon="mdi-star"
            v-model="part.starred"
            :true-value="true"
            :false-value="false"
            color="#fcba03"
            label="Star"
            hide-details
          ></v-switch>
        </v-col>
        <v-col cols="1">
          <v-btn
            v-if="origin_name == ''"
            type="submit"
            @click="newPart"
            text="Add"
          ></v-btn>
          <v-btn v-else type="submit" @click="updatePart" text="Update"></v-btn>
        </v-col>
      </v-row>
      <v-row>
        <v-col>
          <v-text-field
            label="Name"
            variant="outlined"
            v-model="part.name"
            placeholder="RP2040"
            :rules="[rules.required, rules.duplicate]"
            required
          ></v-text-field>
        </v-col>
        <v-col cols="4">
          <v-autocomplete
            label="Category"
            variant="outlined"
            v-model="part.category"
            :items="Object.values(categories).map((cat) => cat.name)"
            :rules="[rules.required]"
          ></v-autocomplete>
        </v-col>
        <v-col cols="3">
          <v-text-field
            label="Quantity"
            variant="outlined"
            v-model.number="qty_input"
            placeholder="15"
            :rules="[rules.required]"
            required
            type="number"
            min="0"
          ></v-text-field>
        </v-col>
      </v-row>

      <v-divider class="my-6"></v-divider>

      <v-row class="mb-n6">
        <v-col>
          <v-autocomplete
            label="Package"
            variant="outlined"
            v-model="part.package"
            :items="Object.values(packages).map((pck) => pck.name)"
            clearable
          ></v-autocomplete>
        </v-col>
        <v-col>
          <v-text-field
            label="Package Detail"
            variant="outlined"
            v-model="part.package_detail"
            placeholder="P0.4mm EP3.2x3.2mm"
          ></v-text-field>
        </v-col>
        <v-col>
          <v-autocomplete
            label="Manufacturer"
            variant="outlined"
            v-model="part.mfr"
            :items="Object.values(mfrs).map((mfr) => mfr.name)"
            clearable
          ></v-autocomplete>
        </v-col>
        <v-col>
          <v-text-field
            label="Mfr #"
            variant="outlined"
            v-model="part.mfr_no"
            placeholder="SC0914(7)"
            hint="Manufacturer part number"
          ></v-text-field>
        </v-col>
      </v-row>
      <v-row class="mb-n6">
        <v-col>
          <v-text-field
            label="Alias"
            variant="outlined"
            v-model="part.alias"
            placeholder=""
          ></v-text-field>
        </v-col>
        <v-col>
          <v-text-field
            label="Location"
            variant="outlined"
            v-model="part.location"
            placeholder="Box #1"
            hint="Storage location"
          ></v-text-field>
        </v-col>
      </v-row>
      <v-row class="mb-n6">
        <v-col>
          <v-text-field
            label="Product Link"
            variant="outlined"
            v-model="part.product_link"
            placeholder="https://"
          ></v-text-field>
        </v-col>
        <v-col>
          <v-text-field
            label="Datasheet Link"
            variant="outlined"
            v-model="part.datasheet_link"
            placeholder="https://"
          ></v-text-field>
        </v-col>
        <v-col>
          <v-text-field
            label="Image Link"
            variant="outlined"
            v-model="part.image_link"
            placeholder="https:// or image.jpg"
            hint="Enter a URL or a filename from the assets folder"
          ></v-text-field>
        </v-col>
      </v-row>
      <v-row class="mb-n6">
        <v-col>
          <v-textarea
            label="Description"
            variant="outlined"
            v-model="part.description"
            placeholder="Write something ..."
          ></v-textarea>
        </v-col>
      </v-row>

      <v-divider class="my-6"></v-divider>

      <v-row>
        <v-col>
          <v-card title="Custom Fields" variant="text">
            <v-spacer class="my-2"></v-spacer>
            <PartCustomField
              v-for="cf in custom_fields"
              :current="cf"
              :existing="undefined"
              @del="handleCustomFieldDel"
            />
            <PartCustomField
              :current="new_custom_field"
              :existing="custom_fields"
              @update="handleCustomFieldUpdate"
              @add="handleCustomFieldAdd"
            />
          </v-card>
        </v-col>
      </v-row>

      <v-row>
        <v-col>
          <v-card title="Suppliers" variant="text">
            <v-spacer class="my-2"></v-spacer>
            <PartSupplier
              v-for="s in suppliers"
              :current="s"
              :existing="undefined"
              @del="handleSupplierDel"
            />
            <PartSupplier
              :current="new_supplier"
              :existing="suppliers"
              @update="handleSupplierUpdate"
              @add="handleSupplierAdd"
            />
          </v-card>
        </v-col>
      </v-row>
    </v-container>

    <v-snackbar v-model="snackbar">
      {{ snackbar_msg }}
      <template v-slot:actions>
        <v-btn color="pink" variant="text" @click="snackbar = false">
          Close
        </v-btn>
      </template>
    </v-snackbar>
  </v-form>
</template>
