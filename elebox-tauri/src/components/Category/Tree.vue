<script setup lang="ts">
import { onMounted, ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { TreeNode } from "@/types/category";
import TreeItem from "@/components/Category/TreeItem.vue";

const tree_nodes = ref<TreeNode[]>([]);

async function getTreeNodes() {
  tree_nodes.value = await invoke("get_tree");
}

onMounted(getTreeNodes);
</script>

<template>
  <TreeItem v-if="tree_nodes.length > 0" :nodes="tree_nodes" />
</template>
