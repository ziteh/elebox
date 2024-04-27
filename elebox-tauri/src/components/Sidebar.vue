<script setup lang="ts">
</script>

<template>
    <div class="menu">
        <router-link to="/" class="button">
            <span class="text">Home</span>
        </router-link>
        <router-link to="/part" class="button">
            <span class="text">Part</span>
        </router-link>
        <router-link to="/category" class="button">
            <span class="text">Category</span>
        </router-link>

        <div class="flex"></div>

        <router-link to="/settings" class="button">
            <span class="text">Setting</span>
        </router-link>
    </div>
</template>

<style>
.menu {
    display: flex;
    flex-direction: column;
    background-color: var(--primary);

    .flex {
        flex: 1 1 0%;
    }
}
</style>