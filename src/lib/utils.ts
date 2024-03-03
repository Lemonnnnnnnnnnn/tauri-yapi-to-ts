import { invoke } from "@tauri-apps/api/tauri";
import type { SuccessResponse } from "./types/public";
import { toast } from "@zerodevx/svelte-toast";
import { toastTheme } from "./consts";
import { processingModalOpen, processingModalTotal } from "./store";

export function request(name: string, data?: Record<keyof any, any>): any {
    let json = JSON.stringify(data);
    return invoke(name, { data: json });
}

export function isEmpty(obj: Record<keyof any, any>) {
    for (const prop in obj) {
        if (Object.hasOwn(obj, prop)) {
            return false;
        }
    }

    return true;
}

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

export function startTask(){ 
    invoke<SuccessResponse<number>>('start_task').then((res) => {
        toast.push(res.message, toastTheme.success);
        processingModalOpen.update(() => true);
        console.log(res);
        
        processingModalTotal.update(() => res.data);
    });

}