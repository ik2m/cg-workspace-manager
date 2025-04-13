<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { open } from '@tauri-apps/plugin-dialog';


const pathTree = ref<[]>([]);
const path = ref<string>("");
const errMsg = ref<string>("");


async function openFolderDialog() {
  const selected = await open({
    directory: true,
    multiple: false,
    title: 'フォルダを選択してください',
  });

  if (typeof selected === 'string') {
    path.value = selected;
  }
}

function setPath() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  invoke("list_files_in_dir", { dir: path.value })
      .then((res:object) => {
        pathTree.value = res;
      })
      .catch((err) => {
        errMsg.value = "Error: " + err;
      })
}
</script>

<template>
  <main class="container">
    <h1>hoge</h1>

    <button @click="openFolderDialog">フォルダを選択</button>
    <p>選択したフォルダ: {{ path }}</p>
    <button @click="setPath">決定</button>
    {{ pathTree }}
    {{ errMsg }}
  </main>
</template>

<style scoped>

</style>