<script setup lang="ts">
import { ref } from "vue";
import { PathTree } from "../types";
import PathTreeItem from "./PathTreeItem.vue";
import MaterialIcon from "./MaterialIcon.vue";

withDefaults(defineProps<{
  pathTree: PathTree;
  depth?: number;
}>(),{
  depth: 0
})

const showChildren = ref<boolean>(false);

const toggleChildren = () => {
  showChildren.value = !showChildren.value;
};
</script>

<template>
  <button @click="toggleChildren" class="btn btn-ghost text-md cursor-pointer flex items-center">
    <span :style="{ 'width': `${depth * 8}px` }" />
    <MaterialIcon v-if="pathTree.type === 'dir'" :name="showChildren ? 'keyboard_arrow_down': 'keyboard_arrow_right'" />
    {{ pathTree.name }}
  </button>
  <ul v-if="pathTree.type === 'dir' && showChildren">
    <li v-for="child in pathTree.children" :key="child.path">
      <PathTreeItem :path-tree="child" :depth="depth + 1" />
    </li>
    <li v-if="pathTree.children.length === 0">データはありません</li>
  </ul>
</template>

<style scoped>

</style>