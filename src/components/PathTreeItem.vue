<script setup lang="ts">
import { ref } from "vue";
import { PathTree } from "../types";
import PathTreeItem from "./PathTreeItem.vue";
import MaterialIcon from "./MaterialIcon.vue";

defineProps<{
  pathTree: PathTree;
}>()

const showChildren = ref<boolean>(false);

const toggleChildren = () => {
  showChildren.value = !showChildren.value;
};
</script>

<template>
  <div>
    <span @click="toggleChildren" class="text-md">
    <MaterialIcon :name="showChildren ? 'keyboard_arrow_down': 'keyboard_arrow_right'" />
    {{ pathTree.name }}
    </span>
    <div v-if=" pathTree.children && showChildren" class="pl-2">
      <PathTreeItem
          v-for="child in pathTree.children"
          :path-tree="child"
      />
    </div>
  </div>
</template>

<style scoped>

</style>