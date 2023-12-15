<script lang="ts">
    import { toastTheme } from "@/consts";
    import type { SuccessResponse, TypesTree } from "@/types/public";
    import { request, wop } from "@/utils";
    import { toast } from "@zerodevx/svelte-toast";
    import { confirm } from "@tauri-apps/api/dialog";

    export let expanded = false;
    export let name: string;
    export let children: TypesTree[];
    export let full_path: string;
    export let level = 1;

    function toggle() {
        if (children.length == 0) {
            return;
        }
        expanded = !expanded;
    }

    async function update_service(full_path: string) {
        const confirmed = await confirm("操作将重新生成ts文件，是否确定？");

        if (!confirmed) return;

        request("update_request", { full_path: [full_path] })
            .then((res: SuccessResponse<null>) => {
                toast.push(JSON.stringify(res.message), toastTheme.success);
            })
            .catch((e) => {
                toast.push(JSON.stringify(e), toastTheme.error);
            });
    }

  
</script>

<div class:wrapper={level == 2}>
    <div class="flex-inline items-end">
        <button class="flex-inline items-end" on:click={toggle}>
            {#if children.length}
                <img class="icon" src="/directory.svg" alt="directory" />
            {:else}
                <img class="icon" src="/file.svg" alt="file" />
            {/if}
            <button class="node" class:expanded>{name}</button>
        </button>
        {#if children.length}
            <button
                class="flex-inline items-end"
                style="margin-left:1em"
                on:click={() => update_service(full_path)}
            >
                <img
                    class="icon"
                    src="/update.svg"
                    alt="更新类型对应的 Service"
                />
            </button>
        {/if}
    </div>

    {#if expanded}
        <ul transition:wop>
            {#each children as child}
                <li>
                    <svelte:self {...child} level={level + 1} />
                </li>
            {/each}
        </ul>
    {/if}
</div>

<style>
    .wrapper {
        border: 1px solid #78bdcc;
        border-radius: 6px;
        padding: 8px;
        display: inline-block;
    }

    .node {
        padding: 0 0 0 1em;
        background-size: 1em 1em;
        font-size: 20px;
    }

    button {
        background: #fff;
        cursor: pointer;
        border: none;
        padding: 0;
    }

    .expanded {
        font-weight: bold;
    }

    ul {
        padding: 0.2em 0 0 0.5em;
        margin: 0 0 0 0.5em;
        list-style: none;
        border-left: 1px solid #eee;
        transition: all 1s ease-in;
    }

    li {
        padding: 0.2em 0;
    }

    .icon {
        width: 20px;
        height: 20px;
    }
</style>
