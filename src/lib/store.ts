import { writable } from "svelte/store";
import type { Config } from "./types/public";

export let processingModalOpen = writable(false);
export let processingModalTotal = writable(0);
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
export let PreviewModalOpen = writable(false)
export let PreviewModalContent = writable("")