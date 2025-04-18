<script setup lang="ts">
import { computed, ref } from "vue";
import { Dir, File } from "../types";
import PathTreeItem from "./PathTreeSelectorItem.vue";
import MaterialIcon from "./common/MaterialIcon.vue";

const props = defineProps<{
  pathTree: Dir | File;
  depth: number;
  selectedPath: string | null;
}>();

const emits = defineEmits(["select"]);

const showChildren = ref<boolean>(false);

const colorClass = computed(() => {
  return props.selectedPath === props.pathTree.path
    ? "bg-primary text-primary-content"
    : "";
});

const selectPath = () => {
  emits("select", props.pathTree.path);
};
const toggleChildren = () => {
  showChildren.value = !showChildren.value;
};
</script>

<template>
  <li>
    <button
      @click="selectPath"
      @dblclick="toggleChildren"
      class="btn btn-ghost text-md cursor-pointe w-full justify-start rounded-selector"
      :class="colorClass"
    >
      <span class="inline-flex items-center">
        <span :style="{ width: `${depth * 12}px` }" />
        <MaterialIcon
          v-if="pathTree.type === 'dir'"
          name="keyboard_arrow_down"
          class="hover:bg-blue-50/50 rounded-selector transition-all duration-200 -rotate-90"
          :class="{ 'rotate-0': showChildren }"
          @click.stop="toggleChildren"
        />
        <span v-else class="w-[24px]"></span>
        {{ pathTree.name }}
      </span>
    </button>
    <ul v-if="pathTree.type === 'dir' && showChildren">
      <PathTreeItem
        v-for="child in pathTree.children"
        :key="child.path"
        :path-tree="child"
        :depth="depth + 1"
        :selected-path="selectedPath"
        @select="emits('select', $event)"
      />
      <li v-if="pathTree.children.length === 0">データはありません</li>
    </ul>
  </li>
</template>

<style scoped></style>
