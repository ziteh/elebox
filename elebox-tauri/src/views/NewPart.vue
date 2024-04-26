<script setup lang="ts">
import { onMounted, ref, reactive } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import router from '../router.js';

interface Types {
  [index: number]: {
    name: string;
    parent: string;
  }
}

const partName = ref("");
const partQty = ref("");
const partType = ref("");
let types = reactive<Types>({});
const location = ref("");

function goHome() {
  router.replace({ path: "/" })
}

async function newPart() {
  await invoke("part_new", { name: partName.value, qty: parseInt(partQty.value), ptype: partType.value });
}

async function getTypes() {
<<<<<<< HEAD:elebox-tauri/src/views/NewPart.vue
  types = await invoke("get_types", {});
  console.debug(`Types: ${types}`);
=======
  types.value = await invoke("get_types", {});
  console.debug(`Types: ${types.value}`);
>>>>>>> origin/core:elebox/src/views/NewPart.vue
}

onMounted(async () => {
  await getTypes();
});
</script>

<template>
  <div class="container">
    <div>
      <button @click="goHome">&#11013; Back</button>
    </div>

    <div>
      <form>
        <div class="field">
          <button @click="newPart">Save</button>
        </div>

        <div class="row">
          <div class="field">
            <label for="part-name-in">Name</label>
            <input id="part-name-in" v-model="partName" placeholder="LM3670" />
          </div>
          <div class="field">
            <label for="part-qty-in">Quantity</label>
            <input id="part-qty-in" v-model="partQty" placeholder="15" type="number" pattern="[0-9]*" />
          </div>
          <div class="field">
            <label for="part-type-in">Type</label>
            <select v-model="partType">
              <option disabled value="Category">Category</option>
              <option v-for="(t, index) in types" :key="index" :title="t.parent">
                {{ t.name }}
              </option>
            </select>
          </div>
        </div>

        <div class="row">
          <div class="field">
            <label for="part-name-in">Package</label>
            <select v-model="location">
              <option disabled value="Category">Package tyep</option>
              <option>
                SOT-23
              </option>
            </select>
          </div>
          <div class="field">
            <label for="part-name-in">Manufacturer</label>
            <select v-model="location">
              <option disabled value="Category">Manufacturer</option>
              <option>
                Texas Instruments
              </option>
            </select>
          </div>
          <div class="field">
            <label for="part-qty-in">Cost</label>
            <input id="part-qty-in" v-model="location" placeholder="15" type="number" pattern="[0-9]*" />
          </div>
        </div>

        <div class="row">
          <div class="field">
            <label for="part-name-in">Description</label>
            <textarea id="multiLineInput" v-model="location" rows="4" cols="50"
              placeholder="Write something ..."></textarea>
          </div>
          <div class="field">
            <label for="part-name-in">Location</label>
            <input id="part-name-in" v-model="location" placeholder="Box #1" />
          </div>
        </div>

        <div class="row">
          <div class="field">
            <label for="part-name-in" title="Manufacturer part number">Mfr #</label>
            <input id="part-name-in" v-model="location" placeholder="LM3670MF-3.3/NOPB"
              title="Manufacturer part number" />
          </div>
          <div class="field">
            <label for="part-name-in">Mouser #</label>
            <input id="part-name-in" v-model="location" placeholder="" />
          </div>
          <div class="field">
            <label for="part-name-in">Digi-Key #</label>
            <input id="part-name-in" v-model="location" placeholder="" />
          </div>
          <div class="field">
            <label for="part-name-in">Alias</label>
            <input id="part-name-in" v-model="location" placeholder="" />
          </div>
        </div>
        <div class="row">
          <div class="field">
            <label for="part-name-in">Product Url</label>
            <input id="part-name-in" v-model="location" placeholder="" />
          </div>
          <div class="field">
            <label for="part-name-in">Datasheet</label>
            <input id="part-name-in" v-model="location" placeholder="" />
          </div>
          <div class="field">
            <label for="part-name-in">Image</label>
            <input id="part-name-in" v-model="location" placeholder="" />
          </div>
          <div class="field">
            <label for="part-name-in">Octopart</label>
            <input id="part-name-in" v-model="location" placeholder="" />
          </div>
          <div class="field">
            <label for="part-name-in">Suppliers</label>
            <input id="part-name-in" v-model="location" placeholder="" />
          </div>
        </div>
      </form>
    </div>
  </div>
</template>

<style>
.row {
  display: flex;
  justify-content: space-between;
}
</style>
