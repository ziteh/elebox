<script setup lang="ts">
import { onMounted, ref, reactive } from "vue";
import { DbCategory as Db } from "../db_cmd_category";

const props = defineProps<{ origin_name?: string }>();
const category = ref<Db.Category>({ name: "", parent: "", alias: "" });
let categories = reactive<Db.Category[]>([]);

async function add() {
  if (category.value === undefined) {
    console.warn("undefined");
    return;
  }

  await Db.add(category.value);
  await list();
}

async function update() {
  if (props.origin_name === undefined || category.value === undefined) {
    return;
  }

  if (category.value.parent === "") {
    category.value.parent = undefined; // To root
  }

  await Db.update(props.origin_name, category.value);
  await list();
}

async function list() {
  const data = await Db.list();

  if (props.origin_name) {
    data.splice(0, 0, { name: "" }); // Root
  }
  Object.assign(categories, data);

  console.debug(`get categories: ${categories.length}`);
}

async function get(name: string) {
  const data = await Db.get(name);
  category.value = data as Db.Category;
}

onMounted(() => {
  if (props.origin_name !== undefined) {
    get(props.origin_name);
  }

  list();
});
</script>

<template>
  <v-form>
    <v-container>
      <v-row class="ga-8">
        <v-text-field
          label="Name"
          variant="outlined"
          v-model="category.name"
          placeholder="MCU"
          :rules="[(v: any) => !!v || 'Required']"
          required
        ></v-text-field>
        <v-text-field
          label="Alias"
          variant="outlined"
          v-model="category.alias"
          placeholder=""
        ></v-text-field>
        <v-select
          label="Parent"
          :items="Object.values(categories).map((c) => c.name)"
          variant="outlined"
          v-model="category.parent"
        ></v-select>
        <v-btn v-if="props.origin_name === undefined" @click="add">Add</v-btn>
        <v-btn v-else @click="update">Update</v-btn>
      </v-row>
    </v-container>
  </v-form>
</template>
