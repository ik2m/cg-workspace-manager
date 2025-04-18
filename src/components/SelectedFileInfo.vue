<script setup lang="ts">
import { defineProps } from "vue";

defineProps<{
  path: string;
}>();
</script>

<template>
  <div class="bg-accent rounded-md p-1 m-1">
    <span class="line-clamp-1">{{ path }}</span>
    <button class="btn">open</button>
  </div>
</template>
