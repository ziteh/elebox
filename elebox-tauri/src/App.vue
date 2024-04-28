<script setup lang="ts">
</script>

<template>
  <v-app>

    <v-navigation-drawer expand-on-hover rail mobile-breakpoint="sm">
      <v-list density="compact" nav>
        <v-list-item prepend-icon="mdi-folder" title="Home" to="/"></v-list-item>

        <v-divider></v-divider>

        <v-list-item prepend-icon="mdi-account-multiple" title="Parts" to="/part"></v-list-item>
        <v-list-item prepend-icon="mdi-account-multiple" title="Categories" to="category"></v-list-item>
        <v-list-item prepend-icon="mdi-star" title="Settings" to="/settings"></v-list-item>
      </v-list>
    </v-navigation-drawer>

    <v-main>
      <router-view />
    </v-main>

  </v-app>
</template>
