<script setup lang="ts">
import { computed, ref } from "vue";
import { Dir, File } from "../types";
import PathTreeItem from "./PathTreeSelectorItem.vue";
import MaterialIcon from "./MaterialIcon.vue";

const props = defineProps<{
  pathTree: Dir | File;
  depth: number;
  selectedPath: string | null;
}>();

const emits = defineEmits(["select"]);

const showChildren = ref<boolean>(false);

const colorClass = computed(() => {
  return props.selectedPath === props.pathTree.path ? "bg-accent" : "";
});

const onClick = () => {
  showChildren.value = !showChildren.value;
  emits("select", props.pathTree.path);
};
</script>

<template>
  <li>
    <button
      @click="onClick"
      class="btn btn-ghost text-md cursor-pointe w-full justify-start rounded-selector"
      :class="colorClass"
    >
      <span class="inline-flex items-center">
        <span :style="{ width: `${depth * 8}px` }" />
        <MaterialIcon
          v-if="pathTree.type === 'dir'"
          :name="showChildren ? 'keyboard_arrow_down' : 'keyboard_arrow_right'"
        />
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
