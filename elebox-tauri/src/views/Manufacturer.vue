<script setup lang="ts">
import { onMounted, ref, reactive } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

interface Manufacturer {
  [index: number]: {
    name: string;
    alias: string;
    url: string;
  }
}

let manufacturers = reactive<Manufacturer>({});

const name = ref("");
const alias = ref("");
const url = ref("");

// async function newCategory() {
//     await invoke("new_category", { name: catName.value, parent: catParent.value });
//     console.debug(`new category: ${catName.value}, ${catParent.value}`);

//     await getCategories();
// }

// async function getCategories() {
//     const cs = await invoke("get_categories", {});
//     Object.assign(categories, cs);
//     console.debug(`get categories: ${categories}`);
// }

// onMounted(getCategories);
</script>

<template>
    <v-container>
        <v-form fast-fail @submit.prevent>
            <v-container>
                <v-row class="ga-8">
                    <v-text-field label="Name" variant="outlined" v-model="name" placeholder="Texas Instruments" :rules="[(v: any) => !!v || 'Required']" required></v-text-field>
                    <v-text-field label="Alias" variant="outlined" v-model="alias" placeholder="TI"></v-text-field>
                    <v-btn type="submit" @click="">Add</v-btn>
                </v-row>
                <v-row>
                    <v-text-field label="Url" variant="outlined" v-model="url" placeholder="https://"></v-text-field>
                </v-row>
            </v-container>
        </v-form>
    <v-table>
        <thead>
            <tr>
                <th>Name</th>
                <th>Alias</th>
                <th>Edit</th>
            </tr>
        </thead>
        <tbody>
            <tr v-for="(mfr, i) in manufacturers" :key="i">
                <td>{{ mfr.name }}</td>
                <td>{{ mfr.alias }}</td>
                <td>
                </td>
            </tr>
        </tbody>
    </v-table>
    </v-container>
</template>
