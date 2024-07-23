<script setup lang="ts">
import { DbPart as Db } from "../utils/db_cmd_part";

const props = defineProps<{ part: string }>();

async function remove() {
  await Db.remove(props.part);
}
</script>

<template>
  <v-btn
    density="comfortable"
    icon="mdi-trash-can-outline"
    :title="`Delete: ${props.part}`"
    @click="remove"
  ></v-btn>
</template>
