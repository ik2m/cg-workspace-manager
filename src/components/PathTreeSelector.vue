<script setup lang="ts">
import PathTreeSelectorItem from "./PathTreeSelectorItem.vue";
import {PathTree} from "../types";
import {computed} from "vue";

const props = defineProps<{
  pathTree: PathTree;
}>()

const colorClass = computed(()=> {
  return selectedPath.value === props.pathTree.path ? 'bg-accent' : '';
});

const selectedPath = defineModel<string|null>({ default: null })
</script>

<template>
  <div>
    <button class="btn btn-ghost text-md cursor-pointe w-full justify-start" @click="selectedPath=pathTree.path" :class="colorClass">
      {{ pathTree.name }}
    </button>
    <ul v-for="child in pathTree.children" :key="pathTree.path">
      <PathTreeSelectorItem
          v-if="child"
          :path-tree="child"
          :depth="0"
          :selected-path="selectedPath"
          @select="selectedPath = $event"
      />
    </ul>
  </div>
</template>

<style scoped>

</style>