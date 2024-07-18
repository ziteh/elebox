<script setup lang="ts">
import { DbPart as Db } from "../db_cmd_part";

const props = defineProps<{ part: string }>();

async function modify(inc: number) {
  await Db.modifyQty(props.part, inc);
}
</script>

<template>
  <v-btn density="comfortable" icon="mdi-plus" @click="modify(1)"></v-btn>
  <v-btn density="comfortable" icon="mdi-minus" @click="modify(-1)"></v-btn>
</template>
