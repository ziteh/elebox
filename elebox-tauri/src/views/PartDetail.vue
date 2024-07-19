<script setup lang="ts">
import "../styles.css";
import { onMounted, ref } from "vue";
import { useRoute, useRouter } from "vue-router";
import { DbPart } from "../db_cmd_part";
import ItemEditButton from "../components/ItemEditButton.vue";
import PartDel from "../components/PartDel.vue";

const router = useRouter();
const route = useRoute();
const name = ref();
const part = ref<DbPart.Part>({
  name: "",
  quantity: 0,
  category: "",
  custom_fields: [],
  suppliers: [],
  starred: false,
});

async function getPart(name: string) {
  const partData = await DbPart.get(name);
  part.value = partData as DbPart.Part;
  console.log(partData);
}

async function removePart() {
  await DbPart.remove(name.value);
  router.replace("/"); // Back to home
}

onMounted(() => {
  name.value = route.params.name;
  getPart(name.value);
});
</script>

<template>
  <v-container v-if="name">
    <div>
      <v-row>
        <v-col>
          <v-btn :to="{ name: 'home' }">Back</v-btn>
        </v-col>
        <v-col cols="1">
          <ItemEditButton :path_name="'update_part'" :item_name="name" />
        </v-col>
        <v-col cols="1">
          <v-dialog max-width="500">
            <template v-slot:activator="{ props: activatorProps }">
              <v-btn
                v-bind="activatorProps"
                density="comfortable"
                icon="mdi-trash-can-outline"
                title="Delete"
              ></v-btn>
            </template>

            <template v-slot:default="{ isActive }">
              <v-card
                title="Confirm Delete"
                prepend-icon="mdi-trash-can-outline"
              >
                <v-card-text>
                  <p>
                    Are you sure you want to delete this part
                    <strong>{{ part.name }}</strong> ?
                  </p>
                  <p>This action cannot be undone.</p>
                </v-card-text>

                <v-card-actions>
                  <v-spacer></v-spacer>
                  <v-btn
                    text="Cancel"
                    @click="isActive.value = false"
                    variant="tonal"
                  ></v-btn>
                  <v-btn
                    text="Delete"
                    @click="removePart"
                    color="red"
                    variant="tonal"
                  ></v-btn>
                </v-card-actions>
              </v-card>
            </template>
          </v-dialog>
        </v-col>
      </v-row>
    </div>

    <v-divider class="my-4"></v-divider>

    <v-table>
      <thead>
        <tr>
          <th>Name</th>
          <th>
            {{ part.name
            }}<v-icon v-if="part.starred" color="#fcba03" title="Starred" end
              >mdi-star</v-icon
            >
          </th>
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
          <td>Description</td>
          <td>{{ part.description }}</td>
        </tr>
        <tr>
          <td>Manufacturer #</td>
          <td>
            <div v-if="part.mfr_no">
              {{ part.mfr_no }}
              <a
                title="Search on Mouser"
                target="_blank"
                :href="`https://www.mouser.com/c/?q=${part.mfr_no}`"
                ><v-icon>mdi-search-web</v-icon></a
              ><a
                title="Search on Digikey"
                target="_blank"
                :href="`https://www.digikey.com/en/products/result?keywords=${part.mfr_no}`"
                ><v-icon>mdi-search-web</v-icon></a
              ><a
                title="Search on Octopart"
                target="_blank"
                :href="`https://octopart.com/search?q=${part.mfr_no}&currency=USD&specs=0`"
                ><v-icon>mdi-search-web</v-icon></a
              >
            </div>
          </td>
        </tr>
        <tr>
          <td>Manufacturer</td>
          <td>{{ part.mfr }}</td>
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
            <a
              v-if="part.image_link"
              target="_blank"
              :href="part.image_link"
              :title="part.image_link"
            >
              <v-img :src="part.image_link" height="250"></v-img>
            </a>
          </td>
        </tr>
      </tbody>
    </v-table>

    <v-label class="mt-8">Custom Fields</v-label>
    <v-table>
      <thead>
        <tr>
          <th>Name</th>
          <th>Value</th>
          <th>Field Type</th>
        </tr>
      </thead>

      <tbody>
        <tr v-for="cf in part.custom_fields" :key="cf.name">
          <td>{{ cf.name }}</td>
          <td>{{ cf.value }}</td>
          <td>{{ cf.field_type }}</td>
        </tr>
      </tbody>
    </v-table>

    <v-label class="mt-8">Suppliers</v-label>
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
        <tr v-for="s in part.suppliers" :key="s.name">
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
