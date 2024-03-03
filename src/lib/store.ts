import { writable } from "svelte/store";

export let processingModalOpen = writable(false);
export let processingModalTotal = writable(0);
export let runningTask = writable(false)
export let sourcePath = writable("")