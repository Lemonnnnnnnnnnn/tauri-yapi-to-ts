<script lang="ts">
    import { listen } from "@tauri-apps/api/event";
    import Menu from "./components/Menu.svelte";
    import Prompt from "./components/Prompt.svelte";
    import { SvelteToast } from "@zerodevx/svelte-toast";

    let need_init = false;
    let load_project = false;

    listen("missing_config", () => {
        need_init = true;
    });

    listen("init_completed", () => {
        load_project = true;
    });
</script>

<SvelteToast options={{}} />

{#if need_init || load_project}
    <Menu bind:need_init bind:load_project />
{:else}
    <Prompt bind:need_init bind:load_project />
{/if}

<style>
</style>
