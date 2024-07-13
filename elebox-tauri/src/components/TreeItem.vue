<script setup lang="ts">
import { TreeNode } from "../interface";

defineProps({
  nodes: Array as () => TreeNode[],
});
</script>

<template>
  <v-list selectable select-strategy="classic" variant="plain">
    <v-list-item
      color="primary"
      v-for="n in nodes!"
      :key="n.name"
      :title="n.name"
    >
      <TreeItem v-if="n.children.length > 0" :nodes="n.children" />
    </v-list-item>
  </v-list>
</template>
