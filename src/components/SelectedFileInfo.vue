<script setup lang="ts">
import { defineProps } from "vue";
import { openPath } from "@tauri-apps/plugin-opener";

const props = defineProps<{
  path: string;
}>();

const openFile = () => {
  openPath(props.path);
};
</script>

<template>
  <div class="bg-accent rounded-md p-1 m-1">
    <span class="line-clamp-1">{{ props.path }}</span>
    <button class="btn" @click="openFile">open</button>
  </div>
</template>
