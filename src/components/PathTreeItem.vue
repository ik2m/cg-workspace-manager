<script setup lang="ts">
import { ref } from "vue";
import { Dir, File } from "../types";
import PathTreeItem from "./PathTreeItem.vue";
import MaterialIcon from "./MaterialIcon.vue";

withDefaults(defineProps<{
  pathTree: Dir|File;
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
  <li>
    <button @click="toggleChildren" class="btn btn-ghost text-md cursor-pointe w-full justify-start">
      <span class="inline-flex items-center">
      <span :style="{ 'width': `${depth * 8}px` }" />
      <MaterialIcon v-if="pathTree.type === 'dir'" :name="showChildren ? 'keyboard_arrow_down': 'keyboard_arrow_right'" />
      {{ pathTree.name }}
      </span>
    </button>
    <ul v-if="pathTree.type === 'dir' && showChildren">
      <PathTreeItem v-for="child in pathTree.children" :key="child.path" :path-tree="child" :depth="depth + 1" />
      <li v-if="pathTree.children.length === 0">データはありません</li>
    </ul>
  </li>
</template>

<style scoped>

</style>