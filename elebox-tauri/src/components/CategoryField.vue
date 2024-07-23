<script setup lang="ts">
import { onMounted, ref, reactive } from "vue";
import { DbCategory as Db } from "../utils/db_cmd_category";

const props = defineProps<{ origin_name?: string }>();
const category = ref<Db.Category>({ name: "", parent: undefined, alias: "" });
let categories = reactive<Db.Category[]>([]);

const snackbar = ref(false);
const snackbar_msg = ref("");
const rules = ref({
  required: (v: any) => !!v || "Required",
  duplicate: (v: any) =>
    !categories.some((cat) => cat.name === v) || "Already exists",
});

async function add() {
  if (category.value === undefined) {
    console.warn("undefined");
    return;
  }

  await Db.add(category.value)
    .then(() => {
      snackbar.value = true;
      snackbar_msg.value = "Success";
      list();
    })
    .catch((e) => {
      snackbar.value = true;
      snackbar_msg.value = e;
    });
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

  Object.assign(categories, data);

  // The parent category cannot be itself
  if (props.origin_name) {
    const index = categories.findIndex((s) => s.name === props.origin_name);
    if (index !== -1) {
      categories.splice(index, 1);
    }
  }

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
  <v-form @submit.prevent>
    <v-row class="align-center pb-2">
      <v-col>
        <v-text-field
          label="Name"
          variant="outlined"
          v-model="category.name"
          placeholder="MCU"
          :rules="[rules.required, rules.duplicate]"
          required
        ></v-text-field>
      </v-col>
      <v-col>
        <v-text-field
          label="Alias"
          variant="outlined"
          v-model="category.alias"
          placeholder=""
        ></v-text-field>
      </v-col>
      <v-col>
        <v-select
          label="Parent"
          :items="Object.values(categories).map((c) => c.name)"
          variant="outlined"
          v-model="category.parent"
          clearable
        ></v-select>
      </v-col>
      <v-col cols="auto" class="mb-6">
        <v-btn
          v-if="props.origin_name === undefined"
          @click="add"
          type="submit"
          text="Add"
        ></v-btn>
        <v-btn v-else @click="update" type="submit" text="Update"></v-btn>
      </v-col>
    </v-row>
  </v-form>
  <v-snackbar v-model="snackbar">
    {{ snackbar_msg }}
    <template v-slot:actions>
      <v-btn color="pink" variant="text" @click="snackbar = false">
        Close
      </v-btn>
    </template>
  </v-snackbar>
</template>
