<script setup lang="ts">
import {onMounted, ref} from "vue";
import { invoke } from "@tauri-apps/api/core";
import { open } from '@tauri-apps/plugin-dialog';
import { load } from '@tauri-apps/plugin-store';
import type { PathTree } from "./types";
import PathTreeItem from "./components/PathTreeItem.vue";

const SETTING_KEY_WORKSPACE_DIR = "workspace-dir";

const pathTree = ref<PathTree|null>(null);
const workspaceDir = ref<string>("");
const errMsg = ref<string>("");


async function openFolderDialog() {
  const selected = await open({
    directory: true,
    multiple: false,
    title: 'フォルダを選択してください',
  });

  if (typeof selected === 'string') {
    workspaceDir.value = selected;
  }
}

async function storeWorkspaceDir() {
  const store = await load('store.json', { autoSave: false });
  // 値を保存
  await store.set(SETTING_KEY_WORKSPACE_DIR, { value: workspaceDir.value });
  await store.save();
}

async function getWorkspaceDir() {
  // .settings.dat というストアファイルを作成
  const store = await load('store.json', { autoSave: false });
  // 値を取得
  const {value} = await store.get<{ value: string }>(SETTING_KEY_WORKSPACE_DIR);
  workspaceDir.value = value;
}

function getFiles() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  invoke("list_files_in_dir", { dir: workspaceDir.value })
      .then((res:PathTree) => {
        pathTree.value = res;
      })
      .catch((err) => {
        errMsg.value = "Error: " + err;
      })
}

onMounted(() => {
  getWorkspaceDir();
});
</script>

<template>
  <main class="flex flex-col h-screen">
    <div>
      <h1>CG Workspace Manager</h1>
      <button @click="openFolderDialog" class="btn">フォルダを選択</button>
      <button @click="storeWorkspaceDir" class="btn">決定</button>
      <p>選択したフォルダ: {{ workspaceDir }}</p>
      <button @click="getFiles" class="btn">このディレクトリのファイルを出力する</button>
    </div>
    <div class="bg-base-200 flex-grow-1 overflow-auto">
      <PathTreeItem v-if="pathTree" :path-tree="pathTree" />
    </div>
    {{ errMsg }}
  </main>
</template>
<style scoped>

</style>