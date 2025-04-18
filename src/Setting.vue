<script setup lang="ts">
import { useAppSetting } from "./composables/useAppSetting.ts";
import { open } from "@tauri-apps/plugin-dialog";
import { ref, watch } from "vue";

const { workspaceDirs, updateWorkSpaceDirs } = useAppSetting();

const editingWorkspaceDirs = ref<string[]>([]);

async function openFolderDialog() {
  const selected = await open({
    directory: true,
    multiple: false,
    title: "フォルダを選択してください",
  });

  if (selected) {
    editingWorkspaceDirs.value.push(selected);
  }
}

const remove = (index: number) => {
  editingWorkspaceDirs.value.splice(index, 1);
};

watch(
  () => workspaceDirs.value,
  (newVal) => {
    editingWorkspaceDirs.value = [...newVal];
  },
  { immediate: true },
);

const storeWorkspaceDirs = () => {
  updateWorkSpaceDirs(editingWorkspaceDirs.value);
};
</script>

<template>
  <div>
    <button class="btn" @click="openFolderDialog">＋</button>
    <ul>
      <li v-for="(w, index) in editingWorkspaceDirs">
        {{ w }}
        <button class="btn" @click="remove(index)" type="button">-</button>
      </li>
    </ul>
    <button @click="storeWorkspaceDirs" class="btn">決定</button>
  </div>
</template>

<style scoped></style>
