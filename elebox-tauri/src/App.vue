<script setup lang="ts">
import { computed, onMounted } from "vue";
import { useDisplay } from "vuetify";
import { useI18n } from "vue-i18n";
import { invoke } from "@tauri-apps/api/tauri";

const { xs, sm } = useDisplay();
const isExtraSmall = computed(() => xs.value);
const isSmall = computed(() => sm.value);

const { locale } = useI18n();

async function loadLanguage() {
  locale.value = await invoke("get_language"); // TODO cfg type
}

onMounted(loadLanguage);
</script>

<template>
  <v-app>
    <v-navigation-drawer
      :permanent="!isExtraSmall"
      :expand-on-hover="isSmall"
      :rail="isSmall"
    >
      <v-list density="compact" nav class="d-flex flex-column flex-grow-1">
        <v-list-item
          prepend-icon="mdi-home"
          title="Home"
          :to="{ name: 'home' }"
        ></v-list-item>

        <v-divider class="my-2"></v-divider>

        <v-list-item
          prepend-icon="mdi-archive-plus"
          title="New Part"
          :to="{ name: 'new_part' }"
        ></v-list-item>
        <v-list-item
          prepend-icon="mdi-tag-multiple"
          title="Categories"
          :to="{ name: 'categories' }"
        ></v-list-item>
        <v-list-item
          prepend-icon="mdi-chip"
          title="Packages"
          :to="{ name: 'packages' }"
        ></v-list-item>
        <v-list-item
          prepend-icon="mdi-factory"
          title="Manufacturers"
          :to="{ name: 'mfrs' }"
        ></v-list-item>

        <!-- <v-spacer class="flex-grow-1"></v-spacer> -->

        <!-- <v-list-item
          prepend-icon="mdi-cog"
          title="Settings"
          :to="{ name: 'settings' }"
        ></v-list-item> -->
      </v-list>

      <template v-slot:append>
        <div class="pa-4">
          <v-btn
            v-if="isSmall || isExtraSmall"
            icon="mdi-cog"
            text="Settings"
            variant="text"
            title="Settings"
            :to="{ name: 'settings' }"
          >
          </v-btn>
          <v-btn
            v-else
            block
            prepend-icon="mdi-cog"
            text="Settings"
            variant="tonal"
            :to="{ name: 'settings' }"
          >
          </v-btn>
        </div>
      </template>
    </v-navigation-drawer>

    <v-main>
      <router-view />
    </v-main>
  </v-app>
</template>

<style scoped>
.flex-grow-1 {
  flex-grow: 1;
}
</style>
