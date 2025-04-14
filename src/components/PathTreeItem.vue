<script setup lang="ts">
import { ref } from "vue";
import { PathTree } from "../types";
import PathTreeItem from "./PathTreeItem.vue";

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
    <span @click="toggleChildren">
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