<script lang="ts">
    import { toastTheme } from "@/consts";
    import type { SuccessResponse, TypesTree } from "@/types/public";
    import { request } from "@/utils";
    import { toast } from "@zerodevx/svelte-toast";
    import { onMount } from "svelte";
    import Node from "./Node.svelte";
    import { Button, Search } from "carbon-components-svelte";
    import { confirm } from "@tauri-apps/api/dialog";

    let list: TypesTree[] = [];
    let searchKey = "";

    onMount(() => {
        get_data();
    });

    function get_data() {
        request("get_request_list", { key: searchKey })
            .then((res: SuccessResponse<TypesTree[]>) => {
                list = sort(res.data);
                toast.push(JSON.stringify(res.message), toastTheme.success);
            })
            .catch((e) => {
                toast.push(JSON.stringify(e), toastTheme.error);
            });
    }

    async function update_service() {
        const confirmed = await confirm("操作将重新生成ts文件，是否确定？");

        if (!confirmed) return;

        const full_path = list.map((item) => item.full_path);

        request("update_request", { full_path })
            .then((res: SuccessResponse<null>) => {
                toast.push(JSON.stringify(res.message), toastTheme.success);
            })
            .catch((e) => {
                toast.push(JSON.stringify(e), toastTheme.error);
            });
    }

    function sort(list: TypesTree[]) {
        list.forEach((item) => {
            item.children.sort((a, b) => {
                if (a.children.length) {
                    return -1;
                }
                return 0;
            });
            if (item.children.length) {
                sort(item.children);
            }
        });

        return list;
    }

    
</script>

<main>
    <div class="flex justify-between items-center">
        <div class="header">对应的接口树：</div>
        <Button kind="tertiary" on:click={update_service}>更新所有请求</Button>
    </div>
    <div class="flex items-center" style="margin-top:10px;margin-bottom:10px">
        <Search bind:value={searchKey} />
        <Button kind="secondary" on:click={get_data}>搜索</Button>
    </div>

    {#each list as item}
        <Node {...item} expanded={true} />
    {/each}
</main>

<style>
    main {
        margin-left: 10px;
        max-height: 100%;
        max-width: 100%;
        overflow: hidden;
    }

    .header {
        font-size: 22px;
        font-weight: bold;
    }
</style>
