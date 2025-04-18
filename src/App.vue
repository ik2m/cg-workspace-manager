<script setup lang="ts">
import { ref, watch } from "vue";
import { invoke } from "@tauri-apps/api/core";
import type { PathTree } from "./types";
import { useAppSetting } from "./composables/useAppSetting.ts";
import SelectedFileInfo from "./components/SelectedFileInfo.vue";
import PathTreeSelector from "./components/PathTreeSelector.vue";
import MaterialIcon from "./components/common/MaterialIcon.vue";
import SelectBox from "./components/SelectBox.vue";
import Setting from "./Setting.vue";

const pathTree = ref<PathTree | null>(null);
const errMsg = ref<string>("");

const { currentWorkspaceDir, workspaceDirs, updateCurrentWorkSpaceDir } =
  useAppSetting();

function storeCurrentWorkspaceDir(v: string | null) {
  selectedPath.value = null;
  updateCurrentWorkSpaceDir(v);
}

function getFiles() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  invoke<PathTree>("list_files_in_dir", { dir: currentWorkspaceDir.value })
    .then((res) => {
      pathTree.value = res;
    })
    .catch((err) => {
      errMsg.value = "Error: " + err;
    });
}

const selectedPath = ref<string | null>(null);

watch(
  () => currentWorkspaceDir.value,
  (newVal) => {
    if (newVal) {
      getFiles();
    }
  },
  { immediate: true },
);

const showSettings = ref(false);
</script>

<template>
  <main class="flex flex-col h-screen">
    <div>
      <SelectBox
        :options="workspaceDirs"
        :model-value="currentWorkspaceDir"
        @update:model-value="storeCurrentWorkspaceDir"
      />
      <button
        class="btn btn-ghost text-md cursor-pointer"
        @click="showSettings = !showSettings"
      >
        <MaterialIcon name="settings" />
      </button>
      <Setting v-if="showSettings" />
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
