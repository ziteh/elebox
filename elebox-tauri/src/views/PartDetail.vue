<script setup lang="ts">
import { onMounted, ref } from "vue";
import { useRoute, useRouter } from "vue-router";
import { DbPart } from "../db_cmd_part";
import { invoke } from "@tauri-apps/api/tauri";
import ItemEditButton from "../components/ItemEditButton.vue";
import ItemDeleteButton from "../components/ItemDeleteButton.vue";

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
const default_path = ref("");

async function getDefaultPath() {
  default_path.value = await invoke("get_assets_path", {});
}

const image_link = ref("");

async function getPart(name: string) {
  const partData = await DbPart.get(name);
  part.value = partData as DbPart.Part;
  console.log(partData);

  image_link.value = part.value.image_link ?? "";
  if (!image_link.value.startsWith("http")) {
    image_link.value = "/@fs/" + default_path.value + image_link.value;
  }
}

async function removePart() {
  await DbPart.remove(name.value);
  router.replace("/"); // Back to home
}

onMounted(() => {
  name.value = route.params.name;
  getDefaultPath();
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
        <v-col cols="auto">
          <ItemEditButton :path_name="'update_part'" :item_name="name" />
        </v-col>
        <v-col cols="auto">
          <ItemDeleteButton :name="part.name" @delete="removePart" />
        </v-col>
      </v-row>
    </div>

    <v-divider class="my-4"></v-divider>

    <v-table>
      <thead>
        <tr>
          <th>{{ $t("name") }}</th>
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
          <td>{{ $t("category") }}</td>
          <td>{{ part.category }}</td>
        </tr>
        <tr>
          <td>{{ $t("quantity") }}</td>
          <td>{{ part.quantity }}</td>
        </tr>
        <tr>
          <td>{{ $t("alias") }}</td>
          <td>{{ part.alias }}</td>
        </tr>
        <tr>
          <td>{{ $t("location") }}</td>
          <td>{{ part.location }}</td>
        </tr>
        <tr>
          <td>{{ $t("package") }}</td>
          <td>{{ part.package }}</td>
        </tr>
        <tr>
          <td>{{ $t("package_detail") }}</td>
          <td>{{ part.package_detail }}</td>
        </tr>
        <tr>
          <td>{{ $t("description") }}</td>
          <td>{{ part.description }}</td>
        </tr>
        <tr>
          <td>{{ $t("manufacturer") }}</td>
          <td>{{ part.mfr }}</td>
        </tr>
        <tr>
          <td>{{ $t("manufacturer_no") }}</td>
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
          <td>{{ $t("datasheet") }}</td>
          <td>
            <a target="_blank" :href="part.datasheet_link">{{
              part.datasheet_link
            }}</a>
          </td>
        </tr>
        <tr>
          <td>{{ $t("product_link") }}</td>
          <td>
            <a target="_blank" :href="part.product_link">{{
              part.product_link
            }}</a>
          </td>
        </tr>
        <tr>
          <td>{{ $t("image") }}</td>
          <td>
            <a
              v-if="image_link"
              target="_blank"
              :href="image_link"
              :title="image_link"
            >
              <v-img :src="image_link" height="250"></v-img>
            </a>
          </td>
        </tr>
      </tbody>
    </v-table>

    <v-label class="mt-8">{{ $t("custom_fields") }}</v-label>
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
          <td>
            <a
              v-if="cf.field_type == 'Link'"
              :href="cf.value"
              target="_blank"
              :title="cf.value"
              ><v-icon> mdi-open-in-new </v-icon></a
            >
            <div v-else>
              {{ cf.value }}
            </div>
          </td>
          <td>{{ cf.field_type }}</td>
        </tr>
      </tbody>
    </v-table>

    <v-label class="mt-8">{{ $t("suppliers") }}</v-label>
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
