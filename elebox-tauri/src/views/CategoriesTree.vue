<script setup lang="ts">
import { onMounted, ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import TreeItem from "../components/TreeItem.vue";
import { TreeNode } from "../interface";

const catNodes = ref<TreeNode[]>([]);

async function getCategories() {
  // const cs = await invoke("get_tree", {});
  // console.log(cs);
  // catNodes.value = cs;
  catNodes.value = await invoke("get_tree", {});
}

onMounted(async () => {
  getCategories();
});
</script>

<template>
  <TreeItem v-if="catNodes.length > 0" :nodes="catNodes" />
</template>
