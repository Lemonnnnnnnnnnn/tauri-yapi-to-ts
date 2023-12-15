<script lang="ts">
    import type { SuccessResponse } from "@/types/public";
    import { request } from "@/utils";
    import { onMount } from "svelte";
    import ConfigModal from "./components/ConfigModal.svelte";
    import { toastTheme } from "@/consts";
    import { toast } from "@zerodevx/svelte-toast";
    import TypesTree from "./components/TypesTree.svelte";

    let need_init = false
    let load_types = false

    onMount(() => {
        request("check_request_config")
            .then((res: SuccessResponse<null>) => {
                // toast.push(JSON.stringify(res.message), toastTheme.success);
                load_types = true
            })
            .catch((e) => {
                toast.push(JSON.stringify(e), toastTheme.error);
                need_init = true
            });
    });
</script>

{#if need_init}
    <ConfigModal bind:open={need_init} bind:load_types />
{/if}

{#if load_types}
    <TypesTree />
{/if}

