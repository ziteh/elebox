<script setup lang="ts">
const props = defineProps<{ name: string }>();

const emit = defineEmits(["delete"]);

function deleteItem() {
  if (props.name) {
    emit("delete");
    console.log(`Delete item. ${props.name}`);
  } else {
    console.error(`Delete empty/undefined item. ${props.name}`);
  }
}
</script>

<template>
  <v-dialog max-width="500">
    <template v-slot:activator="{ props: activatorProps }">
      <v-btn
        v-bind="activatorProps"
        density="comfortable"
        icon="mdi-trash-can-outline"
        :title="`Delete: ${props.name}`"
      ></v-btn>
    </template>

    <template v-slot:default="{ isActive }">
      <v-card title="Confirm Delete" prepend-icon="mdi-trash-can-outline">
        <v-card-text>
          <p>
            Are you sure you want to delete
            <strong>{{ props.name }}</strong> ?
          </p>
          <p>This action cannot be undone.</p>
        </v-card-text>

        <v-card-actions>
          <v-spacer></v-spacer>
          <v-btn
            color="red"
            variant="text"
            @click="
              {
                deleteItem();
                isActive.value = false;
              }
            "
          >
            Delete
          </v-btn>
          <v-btn variant="tonal" @click="isActive.value = false">Cancel</v-btn>
        </v-card-actions>
      </v-card>
    </template>
  </v-dialog>
</template>
