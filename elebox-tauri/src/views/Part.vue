<script setup lang="ts">
import { onMounted, ref, reactive } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

interface Categories {
  [index: number]: {
    name: string;
    parent: string;
  }
}

const partName = ref("");
const partQty = ref("");
const category = ref("");
let categories = reactive<Categories>({});

const val = ref("");

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
  <v-form>
    <v-container>
      <v-row>
        <v-btn @click="newPart">Save</v-btn>
      </v-row>
      <v-row>
        <v-text-field label="Name" variant="outlined" v-model="partName" placeholder="RP2040"></v-text-field>
        <v-text-field label="Quantity" variant="outlined" v-model="partQty"></v-text-field>
        <v-select label="Category" :items="Object.values(categories).map(cat => cat.name)"
          variant="outlined"></v-select>
      </v-row>
      <v-row>
        <v-select label="Package" :items="Object.values(categories).map(cat => cat.name)" variant="outlined"></v-select>
        <v-select label="Manufacturer" :items="Object.values(categories).map(cat => cat.name)"
          variant="outlined"></v-select>
        <v-text-field label="Cost" variant="outlined" v-model="val"></v-text-field>
        <v-text-field label="Location" variant="outlined" v-model="val" placeholder="Box #1"></v-text-field>
      </v-row>
      <v-row>
        <v-text-field label="Alias" variant="outlined" v-model="val" placeholder=""></v-text-field>
        <v-text-field label="Mfr #" variant="outlined" v-model="val" placeholder="SC0914(7)"
          title="Manufacturer #"></v-text-field>
        <v-text-field label="Mouser #" variant="outlined" v-model="val" placeholder="358-SC09147"></v-text-field>
        <v-text-field label="Digi-Key #" variant="outlined" v-model="val"
          placeholder="2648-SC0914(7)CT-ND"></v-text-field>
      </v-row>
      <v-row>
        <v-text-field label="Product Url" variant="outlined" v-model="val" placeholder="https://"></v-text-field>
        <v-text-field label="Datasheet" variant="outlined" v-model="val" placeholder="https://"></v-text-field>
        <v-text-field label="Image" variant="outlined" v-model="val" placeholder=""></v-text-field>
      </v-row>
      <v-row>
        <v-textarea label="Description" variant="outlined" v-model="val" placeholder="Write something ..."></v-textarea>
      </v-row>
      <v-row>
        <v-textarea label="Suppliers" variant="outlined" v-model="val" placeholder=""></v-textarea>
      </v-row>
    </v-container>
  </v-form>
</template>