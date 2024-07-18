<script setup lang="ts">
import { onMounted, ref } from "vue";
import { useRoute } from "vue-router";
import { DbPart } from "../db_cmd_part";
import ItemEditButton from "../components/ItemEditButton.vue";
import PartDel from "../components/PartDel.vue";

const route = useRoute();
const name = ref();
const part = ref<DbPart.Part>({
  name: "",
  quantity: 0,
  category: "",
  custom_fields: [],
  suppliers: [],
});

async function getPart(name: string) {
  const partData = await DbPart.get(name);
  part.value = partData as DbPart.Part;
  console.log(partData);
}

onMounted(() => {
  name.value = route.params.name;
  getPart(name.value);
});
</script>

<template>
  <v-container v-if="name">
    <v-btn :to="{ name: 'home' }">Back</v-btn>
    <ItemEditButton :path_name="'update_part'" :item_name="name" />
    <PartDel :part="name" />
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
          <td>
            <a target="_blank" :href="part.datasheet_link">{{
              part.datasheet_link
            }}</a>
          </td>
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
          <td>
            <a target="_blank" :href="part.product_link">{{
              part.product_link
            }}</a>
          </td>
        </tr>
        <tr>
          <td>Image</td>
          <td>
            <v-img
              v-if="part.image_link"
              :src="part.image_link"
              :title="part.image_link"
              height="250"
            ></v-img>
          </td>
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
