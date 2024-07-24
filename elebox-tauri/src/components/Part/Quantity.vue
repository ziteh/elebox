<script setup lang="ts">
import { onMounted, ref } from "vue";
import { DbPart as Db } from "@/utils/db_cmd_part";

const props = defineProps<{ part: string }>();
const qty = ref<number | undefined>(undefined);

async function modifyQty(increment: number) {
  await Db.modifyQty(props.part, increment);
  await fetchQty();
}

async function fetchQty() {
  const data = await Db.get(props.part);
  const part = data as Db.Part;
  qty.value = part.quantity;
}

onMounted(fetchQty);
</script>

<template>
  <v-btn
    density="comfortable"
    variant="text"
    size="small"
    icon="mdi-minus"
    @click="modifyQty(-1)"
  ></v-btn>
  {{ qty }}
  <v-btn
    density="comfortable"
    variant="text"
    size="small"
    icon="mdi-plus"
    @click="modifyQty(1)"
  ></v-btn>
</template>
