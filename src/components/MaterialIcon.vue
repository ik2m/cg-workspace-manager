<script setup lang="ts">
defineProps<{
  name: string
}>()
</script>

<template>
    <span class="material-symbols-rounded">
      {{ name }}
    </span>
</template>

<style scoped>
.material-symbols-rounded {
 font-variation-settings:
     'FILL' 1,
     'wght' 500,
     'GRAD' 0,
     'opsz' 24
}
</style>