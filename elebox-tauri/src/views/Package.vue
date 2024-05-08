<script setup lang="ts">
import { onMounted, ref, reactive } from "vue";
import { invoke } from "@tauri-apps/api/tauri";


interface Package {
    [index: number]: {
        ptype: string;
        name: string;
        alias: string;
    }
}

let packages = reactive<Package>({});

const ptype = ref();
const name = ref();
const alias = ref();

async function newPackage() {
    if (alias.value == undefined) {
        alias.value = ""
    }
    await invoke("new_package", { name: name.value, ptype: ptype.value, alias: alias.value });

    await getPackages();
}

async function delPackage(name: string) {
    await invoke("del_package", { name });
    await getPackages();
}

async function getPackages() {
    const cs = await invoke("get_packages", {});
    Object.assign(packages, cs);
}

onMounted(getPackages);
</script>

<template>
    <v-container>
        <v-form fast-fail @submit.prevent>
            <v-container>
                <v-row class="ga-8">
                    <v-select label="Type" :items="['SMT', 'THT', 'Others']" variant="outlined" v-model="ptype"
                        :rules="[(v: any) => !!v || 'Required']" required></v-select>
                    <v-text-field label="Name" variant="outlined" v-model="name" placeholder="SOT-23"
                        :rules="[(v: any) => !!v || 'Required']" required></v-text-field>
                    <v-text-field label="Alias" variant="outlined" v-model="alias" placeholder=""></v-text-field>
                    <v-btn type="submit" @click="newPackage">Add</v-btn>
                </v-row>
            </v-container>
        </v-form>
        <v-table>
            <thead>
                <tr>
                    <th>Name</th>
                    <th>Type</th>
                    <th>Alias</th>
                    <th>Edit</th>
                </tr>
            </thead>
            <tbody>
                <tr v-for="(pkg, i) in packages" :key="i">
                    <td>{{ pkg.name }}</td>
                    <td>{{ pkg.ptype.toUpperCase() }}</td>
                    <td>{{ pkg.alias }}</td>
                    <td>
                        <v-btn density="comfortable" icon="mdi-trash-can-outline" @click="delPackage(pkg.name)"
                            title="Delete"></v-btn>
                    </td>
                </tr>
            </tbody>
        </v-table>
    </v-container>
</template>
