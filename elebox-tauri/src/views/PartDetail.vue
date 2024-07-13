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
          <td>Cost</td>
          <td>{{ part.cost }}</td>
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
          <td>Datasheet</td>
          <td>{{ part.datasheet_link }}</td>
        </tr>
        <tr>
          <td>Description</td>
          <td>{{ part.description }}</td>
        </tr>
        <tr>
          <td>Digi-Key #</td>
          <td>{{ part.digikey_no }}</td>
        </tr>
        <tr>
          <td>Mouser #</td>
          <td>{{ part.mouser_no }}</td>
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
          <td>Suppliers</td>
          <td>{{ part.suppliers }}</td>
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
  </v-container>
</template>
