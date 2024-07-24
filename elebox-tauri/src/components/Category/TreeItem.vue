<script setup lang="ts">
import { TreeNode } from "@/types/category";

const props = defineProps<{ nodes: TreeNode[] }>();
</script>

<template>
  <v-list selectable select-strategy="classic" variant="plain">
    <v-list-item
      v-for="n in props.nodes"
      :key="n.name"
      :title="n.name"
      color="primary"
    >
      <TreeItem v-if="n.children.length > 0" :nodes="n.children" />
    </v-list-item>
  </v-list>
</template>
