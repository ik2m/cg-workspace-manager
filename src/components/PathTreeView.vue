<script setup lang="ts">
import { PathTree } from "../types";
import PathTreeView from "./PathTreeView.vue";

defineProps<{
  pathTree: PathTree;
}>()
</script>

<template>
  <div>
    {{ pathTree.name }}
    <div v-if="pathTree.children" style="margin-left: 1rem">
      <PathTreeView
          v-for="child in pathTree.children"
          :path-tree="child"
      />
    </div>
  </div>
</template>

<style scoped>

</style>