<script setup lang="ts">
import { computed } from "vue";

const props = defineProps<{
  options: readonly string[];
}>();

const modelValue = defineModel<string | null>({
  default: null,
});

const isSelectNone = computed(() => modelValue.value === null);
</script>

<template>
  <select class="select" v-model="modelValue">
    <option disabled v-if="isSelectNone" :selected="isSelectNone">
      Pick a workspace dir
    </option>
    <option v-for="option in props.options">
      {{ option }}
    </option>
  </select>
</template>

<style scoped></style>
