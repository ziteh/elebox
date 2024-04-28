<script setup lang="ts">
import { onMounted, reactive } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

interface Categories {
    [index: number]: {
        name: string;
        parent: string;
    }
}

let categories = reactive<Categories>({});

async function getCategories() {
    const cs = await invoke("get_categories", {});
    Object.assign(categories, cs)
    console.debug(`get categories: ${categories}`);
}

async function delCategory(name: string) {
    console.debug(`delete: ${name}`);
    await invoke("del_category", { name });
    // await getCategories();
}

onMounted(getCategories);
</script>

<template>
    <table>
        <thead>
            <tr>
                <th>Name</th>
                <th>Parent</th>
                <th>Edit</th>
            </tr>
        </thead>
        <tbody>
            <tr v-for="(c, index) in categories" :key="index">
                <td>{{ c.name }}</td>
                <td>{{ c.parent }}</td>
                <td>
                    <button @click="delCategory(c.name)" title="Delete">🗑️</button>
                </td>
            </tr>
        </tbody>
    </table>
</template>
