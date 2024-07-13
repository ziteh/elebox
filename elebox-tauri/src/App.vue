<script setup lang="ts"></script>

<template>
  <v-app class="pa-4">
    <v-navigation-drawer expand-on-hover rail mobile-breakpoint="sm">
      <v-list density="compact" nav>
        <v-list-item prepend-icon="mdi-home" title="Home" to="/"></v-list-item>

        <v-divider class="pa-1"></v-divider>

        <v-list-item
          prepend-icon="mdi-archive-plus"
          title="Parts"
          to="/part"
        ></v-list-item>
        <v-list-item
          prepend-icon="mdi-tag-multiple"
          title="Categories"
          to="category"
        ></v-list-item>
        <v-list-item
          prepend-icon="mdi-chip"
          title="Package"
          to="package"
        ></v-list-item>
        <v-list-item
          prepend-icon="mdi-factory"
          title="Manufacturer"
          to="mfr"
        ></v-list-item>
        <v-list-item
          prepend-icon="mdi-cog"
          title="Settings"
          to="/settings"
        ></v-list-item>
      </v-list>
    </v-navigation-drawer>

    <v-main>
      <router-view />
    </v-main>
  </v-app>
</template>
