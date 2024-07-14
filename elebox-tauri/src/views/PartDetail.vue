<script setup lang="ts">
import { onMounted, ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { useRoute } from "vue-router";
import { PartData } from "../interface";

const route = useRoute();
const name = ref();
const part = ref<PartData>({});

async function getPart(name: string) {
  const cs = await invoke("get_part", { part: name });
  console.log(cs);
  part.value = cs as PartData;
}

onMounted(() => {
  name.value = route.params.name;
  getPart(name.value);
});
</script>

<template>
  <v-container>
    <v-btn :to="{ name: 'home' }">Back</v-btn>
    <v-table>
      <thead>
        <tr>
          <th>Part</th>
          <th>{{ part.name }}</th>
        </tr>
      </thead>

      <tbody>
        <tr>
          <td>Category</td>
          <td>{{ part.category }}</td>
        </tr>
        <tr>
          <td>Quantity</td>
          <td>{{ part.quantity }}</td>
        </tr>
        <tr>
          <td>Alias</td>
          <td>{{ part.alias }}</td>
        </tr>
        <tr>
          <td>Location</td>
          <td>{{ part.location }}</td>
        </tr>
        <tr>
          <td>Package</td>
          <td>{{ part.package }}</td>
        </tr>
        <tr>
          <td>Package Detail</td>
          <td>{{ part.package_detail }}</td>
        </tr>
        <tr>
          <td>Datasheet</td>
          <td>{{ part.datasheet_link }}</td>
        </tr>
        <tr>
          <td>Description</td>
          <td>{{ part.description }}</td>
        </tr>
        <tr>
          <td>Mfr #</td>
          <td>{{ part.mfr_no }}</td>
        </tr>
        <tr>
          <td>Manufacturer</td>
          <td>{{ part.mfr }}</td>
        </tr>
        <tr>
          <td>Product</td>
          <td>{{ part.product_link }}</td>
        </tr>
        <tr>
          <td>Image</td>
          <td>{{ part.image_link }}</td>
        </tr>
      </tbody>
    </v-table>

    <v-label>Custom Fields</v-label>
    <v-table>
      <thead>
        <tr>
          <th>Name</th>
          <th>Value</th>
          <th>Type</th>
        </tr>
      </thead>

      <tbody>
        <tr v-for="f in part.custom_fields">
          <td>{{ f.name }}</td>
          <td>{{ f.value }}</td>
          <td>{{ f.field_type }}</td>
        </tr>
      </tbody>
    </v-table>

    <v-label>Suppliers</v-label>
    <v-table>
      <thead>
        <tr>
          <th>Name</th>
          <th>Link</th>
          <th>Price</th>
          <th>Note</th>
        </tr>
      </thead>

      <tbody>
        <tr v-for="s in part.suppliers">
          <td>{{ s.name }}</td>
          <td>
            <a :href="s.link" target="_blank" :title="s.link">Link</a>
          </td>
          <td>{{ s.price == undefined ? "" : s.price.toFixed(3) }}</td>
          <td>{{ s.note }}</td>
        </tr>
      </tbody>
    </v-table>
  </v-container>
</template>
