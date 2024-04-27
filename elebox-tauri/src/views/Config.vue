<script setup lang="ts">
import router from '../router.js';
import DbPath from "../components/DbPath.vue";

function toHome() {
    router.replace({ path: "/" })
}
</script>

<template>
    <div class="container">
        <button @click="toHome">&#11013; Back</button>
        <DbPath />
    </div>
</template>
