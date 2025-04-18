<script setup lang="ts">
import { onMounted, ref, watch } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import type { PathTree } from "./types";
import { useAppSetting } from "./composables/useAppSetting.ts";
import SelectedFileInfo from "./components/SelectedFileInfo.vue";
import PathTreeSelector from "./components/PathTreeSelector.vue";

const pathTree = ref<PathTree | null>(null);
const editingWorkspaceDir = ref<string | null>(null);
const errMsg = ref<string>("");

const { workspaceDir, updateWorkSpaceDir } = useAppSetting();

async function openFolderDialog() {
  const selected = await open({
    directory: true,
    multiple: false,
    title: "フォルダを選択してください",
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
    });
}

const selectedPath = ref<string | null>(null);

watch(
  () => workspaceDir.value,
  (newVal) => {
    if (newVal) {
      getFiles();
    }
  },
  { immediate: true },
);

onMounted(() => {
  editingWorkspaceDir.value = workspaceDir.value;
});
</script>

<template>
  <main class="flex flex-col h-screen">
    <div>
      <button @click="openFolderDialog" class="btn">フォルダを選択</button>
      <button @click="storeWorkspaceDir" class="btn">決定</button>
      <p>選択したフォルダ: {{ editingWorkspaceDir }}</p>
    </div>
    <div class="bg-base-200 flex-grow-1 overflow-auto">
      <PathTreeSelector
        v-if="pathTree"
        :path-tree="pathTree"
        v-model="selectedPath"
      />
    </div>
    <SelectedFileInfo v-if="selectedPath" :path="selectedPath" />
  </main>
</template>
<style scoped></style>
