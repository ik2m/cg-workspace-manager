<script setup lang="ts">
import {onMounted, ref, watch} from "vue";
import { invoke } from "@tauri-apps/api/core";
import { open } from '@tauri-apps/plugin-dialog';
import type { PathTree } from "./types";
import PathTreeItem from "./components/PathTreeItem.vue";
import { useAppSetting } from "./composables/useAppSetting.ts";

const pathTree = ref<PathTree|null>(null);
const editingWorkspaceDir = ref<string|null>(null);
const errMsg = ref<string>("");

const { workspaceDir, updateWorkSpaceDir } = useAppSetting();


async function openFolderDialog() {
  const selected = await open({
    directory: true,
    multiple: false,
    title: 'フォルダを選択してください',
  });

  if (selected) {
    editingWorkspaceDir.value = selected;
  }
}

function storeWorkspaceDir() {
  updateWorkSpaceDir(editingWorkspaceDir.value ?? "");
}

function getFiles() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  invoke<PathTree>("list_files_in_dir", { dir: workspaceDir.value })
      .then((res) => {
        pathTree.value = res;
      })
      .catch((err) => {
        errMsg.value = "Error: " + err;
      })
}

watch(
  () => workspaceDir.value,
  (newVal) => {
    if (newVal) {
      getFiles();
    }
  },
  { immediate: true }
);

onMounted(() => {
  editingWorkspaceDir.value = workspaceDir.value;
});
</script>

<template>
  <main class="flex flex-col h-screen">
    <div>
      <h1>CG Workspace Manager</h1>
      <button @click="openFolderDialog" class="btn">フォルダを選択</button>
      <button @click="storeWorkspaceDir" class="btn">決定</button>
      <p>選択したフォルダ: {{ editingWorkspaceDir }}</p>
    </div>
    <div class="bg-base-200 flex-grow-1 overflow-auto">
      <ul v-if="pathTree" v-for="child in pathTree.children" :key="pathTree.path">
        <PathTreeItem v-if="child" :path-tree="child"/>
      </ul>
    </div>
    {{ errMsg }}
  </main>
</template>
<style scoped>

</style>