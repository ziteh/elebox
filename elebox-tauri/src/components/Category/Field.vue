<script setup lang="ts">
import { onMounted, ref, reactive } from "vue";
import { useRouter } from "vue-router";
import { DbCategory as Db } from "@/utils/db_cmd_category";

const props = defineProps<{
  ori_name?: string; // If ori_name undefined: create mode, otherwise edit mode
}>();

const emit = defineEmits(["update"]);
const router = useRouter();

const current = ref<Db.Category>({ name: "", parent: undefined, alias: "" });
const existing = reactive<string[]>([]);

const snackbar = ref(false);
const snackbar_msg = ref("");

const rules = {
  required: (val: any) => !!val || "Required",
  duplicate: (val: any) => !existing.some((i) => i === val) || "Already exists",
};

async function createNew() {
  if (current.value == undefined) {
    return;
  }

  await Db.add(current.value)
    .then(() => {
      snackbar.value = true;
      snackbar_msg.value = "Success";
      emit("update");
    })
    .catch((err) => {
      snackbar.value = true;
      snackbar_msg.value = err;
    });
}

async function updateOriginal() {
  if (props.ori_name == undefined || current.value == undefined) {
    return;
  }

  if (current.value.parent === "") {
    current.value.parent = undefined; // Set to root category
  }

  await Db.update(props.ori_name, current.value);
  await fetchExisting();
  router.go(-1);
}

async function fetchExisting() {
  const data = await Db.list();
  Object.assign(
    existing,
    data.map((i) => i.name)
  );

  console.debug(`Get categories: ${existing.length}`);

  // The parent category cannot be itself, and rules duplicate
  if (props.ori_name) {
    const self_index = existing.findIndex((i) => i === props.ori_name);
    if (self_index !== -1) {
      existing.splice(self_index, 1); // Remove self from existing categories
    }
  }
}

async function fetchCurrent() {
  if (props.ori_name) {
    const data = await Db.get(props.ori_name);
    current.value = data as Db.Category;
  }
}

onMounted(() => {
  fetchCurrent();
  fetchExisting();
});
</script>

<template>
  <v-form @submit.prevent>
    <v-row class="align-center pb-2">
      <v-col>
        <v-text-field
          label="Name"
          variant="outlined"
          v-model="current.name"
          placeholder="MCU"
          :rules="[rules.required, rules.duplicate]"
          required
        ></v-text-field>
      </v-col>
      <v-col>
        <v-text-field
          label="Alias"
          variant="outlined"
          v-model="current.alias"
        ></v-text-field>
      </v-col>
      <v-col>
        <v-select
          label="Parent"
          :items="existing"
          variant="outlined"
          v-model="current.parent"
          clearable
        ></v-select>
      </v-col>
      <v-col cols="auto" class="mb-6">
        <v-btn
          v-if="props.ori_name === undefined"
          type="submit"
          @click="createNew"
          >Add</v-btn
        >
        <v-btn v-else type="submit" @click="updateOriginal">Update</v-btn>
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
