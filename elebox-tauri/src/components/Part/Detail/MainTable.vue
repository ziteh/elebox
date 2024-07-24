<script setup lang="ts">
import { onMounted, ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { DbPart as Db } from "@/utils/db_cmd_part";

const props = defineProps<{
  current: Db.Part;
}>();

const formatted_image_link = ref("");

async function formatImageLink() {
  const assets_dir = await invoke("get_assets_path", {});

  if (props.current.image_link?.startsWith("http")) {
    // URL
    formatted_image_link.value = props.current.image_link;
  } else {
    // Local assets
    formatted_image_link.value =
      "/@fs/" + assets_dir + props.current.image_link;
  }
}

onMounted(formatImageLink);
</script>

<template>
  <v-table>
    <thead>
      <tr>
        <th>{{ $t("name") }}</th>
        <th>
          {{ current.name }}
          <v-icon v-if="current.starred" color="#fcba03" title="Starred" end>
            mdi-star
          </v-icon>
        </th>
      </tr>
    </thead>

    <tbody>
      <tr>
        <td>{{ $t("category") }}</td>
        <td>{{ current.category }}</td>
      </tr>
      <tr>
        <td>{{ $t("quantity") }}</td>
        <td>{{ current.quantity }}</td>
      </tr>
      <tr>
        <td>{{ $t("alias") }}</td>
        <td>{{ current.alias }}</td>
      </tr>
      <tr>
        <td>{{ $t("location") }}</td>
        <td>{{ current.location }}</td>
      </tr>
      <tr>
        <td>{{ $t("package") }}</td>
        <td>{{ current.package }}</td>
      </tr>
      <tr>
        <td>{{ $t("package_detail") }}</td>
        <td>{{ current.package_detail }}</td>
      </tr>
      <tr>
        <td>{{ $t("description") }}</td>
        <td>{{ current.description }}</td>
      </tr>
      <tr>
        <td>{{ $t("manufacturer") }}</td>
        <td>{{ current.mfr }}</td>
      </tr>
      <tr>
        <td>{{ $t("manufacturer_no") }}</td>
        <td>
          <div v-if="current.mfr_no">
            {{ current.mfr_no }}
            <a
              title="Search on Mouser"
              target="_blank"
              :href="`https://www.mouser.com/c/?q=${current.mfr_no}`"
              ><v-icon>mdi-search-web</v-icon>
            </a>
            <a
              title="Search on Digikey"
              target="_blank"
              :href="`https://www.digikey.com/en/products/result?keywords=${current.mfr_no}`"
              ><v-icon>mdi-search-web</v-icon>
            </a>
            <a
              title="Search on Octopart"
              target="_blank"
              :href="`https://octopart.com/search?q=${current.mfr_no}&currency=USD&specs=0`"
              ><v-icon>mdi-search-web</v-icon>
            </a>
          </div>
        </td>
      </tr>
      <tr>
        <td>{{ $t("datasheet") }}</td>
        <td>
          <a target="_blank" :href="current.datasheet_link">
            {{ current.datasheet_link }}
          </a>
        </td>
      </tr>
      <tr>
        <td>{{ $t("product_link") }}</td>
        <td>
          <a target="_blank" :href="current.product_link">
            {{ current.product_link }}
          </a>
        </td>
      </tr>
      <tr>
        <td>{{ $t("image") }}</td>
        <td>
          <a
            v-if="formatted_image_link"
            target="_blank"
            :href="formatted_image_link"
            :title="formatted_image_link"
          >
            <v-img :src="formatted_image_link" height="250"></v-img>
          </a>
        </td>
      </tr>
    </tbody>
  </v-table>
</template>
