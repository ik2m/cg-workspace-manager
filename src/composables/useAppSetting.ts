import { readonly, ref } from "vue";
import { load } from "@tauri-apps/plugin-store";

const SETTING_KEY_WORKSPACE_DIR = "workspace";
const SETTING_KEY_WORKSPACE_DIR_LIST = "workspace-list";

const initialLoaded = ref<boolean>(false);

const currentWorkspaceDir = ref<string | null>(null);
const workspaceDirs = ref<string[]>([]);

export const useAppSetting = () => {
  const init = async () => {
    const store = await load("store.json", { autoSave: false });
    const v = await store.get<{ value: string }>(SETTING_KEY_WORKSPACE_DIR);
    currentWorkspaceDir.value = v?.value ?? null;

    const dirs = await store.get<{ value: string[] }>(
      SETTING_KEY_WORKSPACE_DIR_LIST,
    );
    workspaceDirs.value = dirs?.value ?? [];
    initialLoaded.value = true;
  };
  if (!initialLoaded.value) {
    init();
  }

  const updateCurrentWorkSpaceDir = async (dir: string | null) => {
    const store = await load("store.json", { autoSave: false });
    await store.set(SETTING_KEY_WORKSPACE_DIR, { value: dir });
    await store.save();
    currentWorkspaceDir.value = dir;
  };

  const updateWorkSpaceDirs = async (dirs: string[]) => {
    const store = await load("store.json", { autoSave: false });
    await store.set(SETTING_KEY_WORKSPACE_DIR_LIST, { value: dirs });
    await store.save();
    workspaceDirs.value = dirs;
  };

  return {
    currentWorkspaceDir: readonly(currentWorkspaceDir),
    workspaceDirs: readonly(workspaceDirs),
    updateCurrentWorkSpaceDir,
    updateWorkSpaceDirs,
  };
};
