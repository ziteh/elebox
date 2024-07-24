<script setup lang="ts">
import { onMounted, ref, reactive } from "vue";
import { useRouter } from "vue-router";
import { DbPart } from "@/utils/db_cmd_part";
import { DbCategory } from "@/utils/db_cmd_category";
import { DbPackage } from "@/utils/db_cmd_package";
import { DbManufacturer as DbMfr } from "@/utils/db_cmd_manufacturer";
import { CustomField, Supplier } from "@/types/part";
import PartCustomField from "@/components/PartCustomField.vue";
import PartSupplier from "@/components/PartSupplier.vue";

const props = defineProps<{
  ori_name?: string; // If ori_name undefined: create mode, otherwise edit mode
}>();

const show_notify = ref(false);
const notify_message = ref("");

const rules = {
  required: (val: any) => !!val || "Required",
  duplicate: (val: any) =>
    !existing.some((part) => part === val) || "Already exists",
};

const router = useRouter();

const current = ref<DbPart.Part>({
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

const existing = reactive<string[]>([]);
const categories = reactive<string[]>([]);
const packages = reactive<string[]>([]);
const mfrs = reactive<string[]>([]);

const custom_fields = reactive<CustomField[]>([]);
const suppliers = reactive<Supplier[]>([]);

function normalizePart(): boolean {
  if (qty_input.value == undefined) {
    return false;
  }

  current.value.quantity = Math.trunc(qty_input.value);

  if (
    !current.value.name ||
    !current.value.category ||
    current.value.quantity < 0
  ) {
    return false;
  }

  if (current.value.mfr === "") {
    current.value.mfr = undefined;
  }
  if (current.value.package === "") {
    current.value.package = undefined;
  }

  current.value.custom_fields = custom_fields;
  current.value.suppliers = suppliers;

  return true; // ok
}

async function createNew() {
  if (!normalizePart()) {
    return;
  }

  await DbPart.add(current.value)
    .then(() => router.replace("/")) // Back to home
    .catch((err) => {
      show_notify.value = true;
      notify_message.value = err;
    });
}

async function updateOriginal() {
  if (!props.ori_name || !normalizePart()) {
    return;
  }

  await DbPart.update(props.ori_name, current.value)
    .then(() => router.replace("/")) // Back to home
    .catch((err) => {
      show_notify.value = true;
      notify_message.value = err;
    });
}

async function fetchParts() {
  const data = await DbPart.list();
  Object.assign(
    existing,
    data.map((p) => p.name)
  );

  if (props.ori_name) {
    const index = existing.findIndex((part) => part === props.ori_name);
    if (index !== -1) {
      existing.splice(index, 1); // Remove self
    }
  }
}

async function fetchCategories() {
  const data = await DbCategory.list();
  Object.assign(
    categories,
    data.map((c) => c.name)
  );
}

async function fetchMfrs() {
  const date = await DbMfr.list();
  Object.assign(
    mfrs,
    date.map((m) => m.name)
  );
}

async function fetchPackages() {
  const data = await DbPackage.list();
  Object.assign(
    packages,
    data.map((p) => p.name)
  );
}

async function fetchCurrent() {
  if (!props.ori_name) {
    return;
  }

  const data = await DbPart.get(props.ori_name);
  current.value = data as DbPart.Part;
  qty_input.value = current.value.quantity;
  Object.assign(custom_fields, current.value.custom_fields);
  Object.assign(suppliers, current.value.suppliers);
}

function handleCustomFieldUpdate(data: { new: CustomField }) {
  new_custom_field.value = data.new;
}

function handleCustomFieldAdd(data: { new: CustomField }) {
  custom_fields.push(data.new);
}

function handleCustomFieldDel(data: { name: string }) {
  const index = custom_fields.findIndex((f) => f.name === data.name); // Find by name
  if (index !== -1) {
    custom_fields.splice(index, 1); // Remove
  }
}

function handleSupplierUpdate(data: { new: Supplier }) {
  new_supplier.value = data.new;
}

function handleSupplierAdd(data: { new: Supplier }) {
  suppliers.push(data.new);
}

function handleSupplierDel(data: { name: string }) {
  const index = suppliers.findIndex((s) => s.name === data.name); // Find by name
  if (index !== -1) {
    suppliers.splice(index, 1); // Remove
  }
}

onMounted(() => {
  fetchCurrent();
  fetchParts();
  fetchCategories();
  fetchMfrs();
  fetchPackages();
});
</script>

<template>
  <v-form @submit.prevent>
    <v-container>
      <v-row align="center">
        <v-spacer></v-spacer>
        <v-col cols="auto">
          <v-switch
            true-icon="mdi-star"
            v-model="current.starred"
            :true-value="true"
            :false-value="false"
            color="#fcba03"
            label="Star"
            hide-details
          ></v-switch>
        </v-col>
        <v-col cols="1">
          <v-btn v-if="props.ori_name" type="submit" @click="updateOriginal">
            Update
          </v-btn>
          <v-btn v-else type="submit" @click="createNew">Add</v-btn>
        </v-col>
      </v-row>
      <v-row>
        <v-col>
          <v-text-field
            label="Name"
            variant="outlined"
            v-model="current.name"
            placeholder="RP2040"
            :rules="[rules.required, rules.duplicate]"
            required
          ></v-text-field>
        </v-col>
        <v-col cols="4">
          <v-autocomplete
            label="Category"
            variant="outlined"
            v-model="current.category"
            :items="categories"
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
            v-model="current.package"
            :items="packages"
            clearable
          ></v-autocomplete>
        </v-col>
        <v-col>
          <v-text-field
            label="Package Detail"
            variant="outlined"
            v-model="current.package_detail"
            placeholder="P0.4mm EP3.2x3.2mm"
          ></v-text-field>
        </v-col>
        <v-col>
          <v-autocomplete
            label="Manufacturer"
            variant="outlined"
            v-model="current.mfr"
            :items="mfrs"
            clearable
          ></v-autocomplete>
        </v-col>
        <v-col>
          <v-text-field
            label="Mfr #"
            variant="outlined"
            v-model="current.mfr_no"
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
            v-model="current.alias"
            placeholder=""
          ></v-text-field>
        </v-col>
        <v-col>
          <v-text-field
            label="Location"
            variant="outlined"
            v-model="current.location"
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
            v-model="current.product_link"
            placeholder="https://"
          ></v-text-field>
        </v-col>
        <v-col>
          <v-text-field
            label="Datasheet Link"
            variant="outlined"
            v-model="current.datasheet_link"
            placeholder="https://"
          ></v-text-field>
        </v-col>
        <v-col>
          <v-text-field
            label="Image Link"
            variant="outlined"
            v-model="current.image_link"
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
            v-model="current.description"
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

    <v-snackbar v-model="show_notify">
      {{ notify_message }}
      <template v-slot:actions>
        <v-btn
          icon="mdi-close"
          density="comfortable"
          variant="text"
          @click="show_notify = false"
        ></v-btn>
      </template>
    </v-snackbar>
  </v-form>
</template>
