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
    <button @click="toggleChildren" class="text-md cursor-pointer inline-flex items-center">
    <MaterialIcon v-if="pathTree.type === 'dir'" :name="showChildren ? 'keyboard_arrow_down': 'keyboard_arrow_right'" />
    {{ pathTree.name }}
    </button>
    <div v-if="pathTree.type === 'dir' && showChildren" class="pl-2">
      <template v-if="pathTree.children.length > 0">
        <PathTreeItem
            v-for="child in pathTree.children"
            :path-tree="child"
        />
      </template>
      <span v-else>データはありません</span>
    </div>
  </div>
</template>

<style scoped>

</style>