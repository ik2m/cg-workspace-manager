<script setup lang="ts">
import PathTreeSelectorItem from "./PathTreeSelectorItem.vue";
import {PathTree} from "../types";

defineProps<{
  pathTree: PathTree;
}>()
</script>

<template>
  <div>
    <button class="btn btn-ghost text-md cursor-pointe w-full justify-start">
      {{ pathTree.name }}
    </button>
    <ul v-for="child in pathTree.children" :key="pathTree.path">
      <PathTreeSelectorItem v-if="child" :path-tree="child"/>
    </ul>
  </div>
</template>

<style scoped>

</style>