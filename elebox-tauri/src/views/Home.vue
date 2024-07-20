<script setup lang="ts">
import { onMounted, ref, reactive } from "vue";
import { useRouter, useRoute } from "vue-router";
import PartList from "../components/PartList.vue";

// onMounted();
</script>

<template>
  <v-container>
    <v-row>
      <v-container>
        <PartList />
      </v-container>
    </v-row>
  </v-container>
</template>
