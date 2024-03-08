import { invoke } from "@tauri-apps/api/tauri";
import type { Config, SuccessResponse } from "./types/public";
import { toast } from "@zerodevx/svelte-toast";
import { toastTheme } from "./consts";
import { config, processingModalOpen, processingModalTotal } from "./store";


function getComputedStyle(node?: HTMLUListElement) {
    return {
        height: node?.offsetHeight,
        width: node?.offsetWidth,
    };
}

export function wop(node?: HTMLUListElement, params?: { duration?: number }) {
    const { height, width } = getComputedStyle(node);
    const { duration = 300 } = params || {};

    return {
        duration,
        css: (t: number) => `
          clip-path: polygon(0 0, ${t * 100}% 0, ${t * 100}% ${t * 100}%, 0 ${t * 100
            }%);
          margin-right: calc((${t - 1})*${width}px);
          margin-bottom: calc((${t - 1})*${height}px);
          overflow-y:hidden
      `,
    };
}

export function startTask() {
    invoke<SuccessResponse<number>>('start_task').then((res) => {
        toast.push(res.message, toastTheme.success);
        processingModalOpen.update(() => true);
        processingModalTotal.update(() => res.data);
    });
}

export function loadConfig(sourcePath: string) {
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