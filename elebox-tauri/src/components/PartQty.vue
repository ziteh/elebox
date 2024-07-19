<script setup lang="ts">
import { onMounted, ref } from "vue";
import { DbPart as Db } from "../db_cmd_part";

const props = defineProps<{ part: string }>();
const qty = ref<number | undefined>(undefined);

async function modifyQty(inc: number) {
  await Db.modifyQty(props.part, inc);
  await refreshQty();
}

async function refreshQty() {
  const data = await Db.get(props.part);
  const part = data as Db.Part;
  qty.value = part.quantity;
}

onMounted(refreshQty);
</script>

<template>
  <v-btn
    density="comfortable"
    variant="text"
    icon="mdi-minus"
    size="small"
    @click="modifyQty(-1)"
  ></v-btn>
  {{ qty }}
  <v-btn
    density="comfortable"
    variant="text"
    icon="mdi-plus"
    size="small"
    @click="modifyQty(1)"
  ></v-btn>
</template>
