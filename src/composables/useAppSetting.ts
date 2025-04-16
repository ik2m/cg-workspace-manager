import {readonly, ref} from 'vue';
import { load } from '@tauri-apps/plugin-store';

const SETTING_KEY_WORKSPACE_DIR = "workspace-dir";

const store = await load('store.json', { autoSave: false });
const initialLoaded = ref<boolean>(false);

const workspaceDir = ref<string|null>(null);



export const useAppSetting = () => {
    if (!initialLoaded.value) {
        store.get<{ value: string }>(SETTING_KEY_WORKSPACE_DIR)
        .then((data) => {
            if(data){
                workspaceDir.value = data.value;
            }
            initialLoaded.value = true;
        })
    }

    const updateWorkSpaceDir = async (dir: string) => {
        await store.set(SETTING_KEY_WORKSPACE_DIR, { value: dir });
        await store.save();
        workspaceDir.value = dir;
    }

    return {
        workspaceDir: readonly(workspaceDir),
        updateWorkSpaceDir,
    };
}