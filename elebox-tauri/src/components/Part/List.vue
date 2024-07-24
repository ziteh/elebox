<script setup lang="ts">
import { onMounted, ref, reactive, computed } from "vue";
import { useRouter, useRoute } from "vue-router";
import { useDisplay } from "vuetify";
import { useI18n } from "vue-i18n";
import { DbPart as Db } from "@/utils/db_cmd_part";
import PartQty from "@/components/Part/Quantity.vue";

const { t } = useI18n();
const router = useRouter();
const route = useRoute();
const { smAndDown } = useDisplay();

const search = ref("");
const headers = ref([
  { key: "name", title: t("name"), sortable: true },
  { key: "alias", title: t("alias"), sortable: true },
  { key: "quantity", title: t("quantity"), sortable: true },
  { key: "category", title: t("category"), sortable: true },
  { key: "package", title: t("package"), sortable: true },
  { key: "mfr", title: t("mfr"), sortable: true },
]);

const responsive_headers = computed(() => {
  if (smAndDown.value) {
    return headers.value.filter((h) => h.key !== "alias");
  }
  return headers.value;
});

const existing = reactive<Db.Part[]>([]);

async function fetchExisting() {
  let promise = await Db.list();
  Object.assign(existing, promise);
}

function reloadPage() {
  // window.location.reload();

  const currentPath = route.path;
  router.replace("/redirect").then(() => {
    router.replace(currentPath);
  });
}

onMounted(fetchExisting);
</script>

<template>
  <v-card flat variant="text">
    <template v-slot:text>
      <v-row class="align-center">
        <v-col>
          <v-text-field
            v-model="search"
            label="Search"
            prepend-inner-icon="mdi-magnify"
            variant="outlined"
            hide-details
            single-line
          ></v-text-field>
        </v-col>
        <v-col cols="auto">
          <v-btn
            text="Update"
            icon="mdi-refresh"
            density="comfortable"
            size="large"
            title="Refresh"
            @click="reloadPage"
          ></v-btn>
        </v-col>
      </v-row>
    </template>

    <v-data-table
      v-if="existing.length > 0"
      :headers="responsive_headers"
      :items="existing"
      :search="search"
    >
      <template v-slot:item.name="{ item }">
        <v-btn
          variant="text"
          :to="{ name: 'part_detail', params: { name: item.name } }"
        >
          {{ item.name }}
          <v-icon v-if="item.starred" color="#fcba03">mdi-star</v-icon>
        </v-btn>
      </template>
      <template v-slot:item.quantity="{ item }">
        <PartQty :part="item.name" />
      </template>
    </v-data-table>
  </v-card>
</template>
