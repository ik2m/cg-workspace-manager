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
    <div v-if="pathTree.children" class="pl-2">
      <PathTreeView
          v-for="child in pathTree.children"
          :path-tree="child"
      />
    </div>
  </div>
</template>

<style scoped>

</style>