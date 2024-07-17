<script setup lang="ts">
import { onMounted, reactive } from "vue";
import { Part } from "../interface";
import { useRouter, useRoute } from "vue-router";
import { DbPart } from "../db_cmd_part";
import PartQty from "../components/PartQty.vue";
import PartDel from "../components/PartDel.vue";
import ItemEditButton from "../components/ItemEditButton.vue";

let parts = reactive<Part[]>([]);

const router = useRouter();
const route = useRoute();

async function getParts() {
  let promise = await DbPart.list();
  Object.assign(parts, promise);
}

function reload() {
  // window.location.reload();

  const currentPath = route.path;
  router.replace("/redirect").then(() => {
    router.replace(currentPath);
  });
}

onMounted(getParts);
</script>

<template>
  <v-container>
    <v-row class="ga-8" align="center">
      <v-autocomplete
        label="Search"
        variant="outlined"
        :items="parts.map((part) => part.name)"
      ></v-autocomplete>
      <v-btn @click="reload">Update</v-btn>
    </v-row>

    <v-table>
      <thead>
        <tr>
          <th>Part</th>
          <th>Quantity</th>
          <th>Category</th>
          <th>Package</th>
          <th>Mfr</th>
          <th>Edit</th>
        </tr>
      </thead>

      <tbody>
        <tr v-for="(p, index) in parts" :key="index">
          <td>
            <v-btn :to="{ name: 'part_detail', params: { name: p.name } }">
              {{ p.name }}</v-btn
            >
          </td>
          <td>
            {{ p.quantity }}
            <PartQty :part="p.name" />
          </td>
          <td>{{ p.category }}</td>
          <td>{{ p.package }}</td>
          <td>{{ p.mfr }}</td>
          <td>
            <!-- <v-btn
              density="comfortable"
              icon="mdi-square-edit-outline"
              :to="{ name: 'update_part', params: { ori_name: p.name } }"
            ></v-btn> -->

          <ItemEditButton
            :path_name="'update_part'"
            :item_name="p.name"
          />
            <PartDel :part="p.name" />
          </td>
        </tr>
      </tbody>
    </v-table>
  </v-container>
</template>
