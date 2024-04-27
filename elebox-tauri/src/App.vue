<script setup lang="ts">
import Sidebar from "./components/Sidebar.vue";
</script>

<template>
  <div class="app">

    <Sidebar />
    <router-view />
  </div>
</template>

<style>
.app {
  display: flex;
  flex-direction: row;
}
</style>
