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

async function getTypes() {
    const cs = await invoke("get_categories", {});
    Object.assign(categories, cs)
    console.debug(`get categories: ${categories}`);
}

onMounted(getTypes);
</script>

<template>
    <ul v-for="(t) in categories">
        <li>{{ t.name }}</li>
    </ul>
</template>
