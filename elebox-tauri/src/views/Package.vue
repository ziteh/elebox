<script setup lang="ts">
import { onMounted, ref, reactive } from "vue";
import { invoke } from "@tauri-apps/api/tauri";


interface Package {
    [index: number]: {
        type: string;
        name: string;
        alias: string;
    }
}

let packages = reactive<Package>({});

const type = ref();
const name = ref();
const alias = ref();

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
                    <v-select label="Type" :items="['SMT', 'THT', 'Others']" variant="outlined" v-model="type"
                        :rules="[(v: any) => !!v || 'Required']" required></v-select>
                    <v-text-field label="Name" variant="outlined" v-model="name" placeholder="SOT-23"
                        :rules="[(v: any) => !!v || 'Required']" required></v-text-field>
                    <v-text-field label="Alias" variant="outlined" v-model="alias" placeholder=""></v-text-field>
                    <v-btn type="submit" @click="">Add</v-btn>
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
                <tr v-for="(pkg, i) in packages" :key="i">
                    <td>{{ pkg.name }}</td>
                    <td>{{ pkg.alias }}</td>
                    <td>
                    </td>
                </tr>
            </tbody>
        </v-table>
    </v-container>
</template>
