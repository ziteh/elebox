<script setup lang="ts">
import { onMounted, reactive } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

interface Types {
    [index: number]: {
        name: string;
        parent: string;
    }
}

let types = reactive<Types>({});

async function getTypes() {
    types = await invoke("get_types", {});
    console.debug(`Types: ${types}`);
}

onMounted(getTypes);
</script>

<template>
    <ul v-for="(t) in types">
        <li>{{ t.name }}</li>
    </ul>
</template>
