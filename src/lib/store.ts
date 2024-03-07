import { writable } from "svelte/store";
import type { Config, SuccessResponse } from "./types/public";
import { invoke } from "@tauri-apps/api";
import { toast } from "@zerodevx/svelte-toast";
import { toastTheme } from "./consts";

export let processingModalOpen = writable(false);
export let processingModalTotal = writable(0);
export let runningTask = writable(false)
export let sourcePath = writable("")
export let config = writable<Config>({
    base_url: "",
    types_path: "",
    project_list: [],
    request_path: "",
    request_template: "",
    header_template: "",
    file_name_template: "",
    type_import_template: "",
})

export function loadProject(sourcePath: string) {
    return invoke<SuccessResponse<Config>>('load_project_config', { sourcePath })
        .then((res) => {
            toast.push(res.message, toastTheme.success);
            config.set(res.data);
            return res
        })
        .catch((e) => {
            toast.push(JSON.stringify(e), toastTheme.error);
        });
}