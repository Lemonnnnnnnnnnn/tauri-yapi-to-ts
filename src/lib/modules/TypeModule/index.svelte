<script lang="ts">
    import ConfigModal from "./components/ConfigModal.svelte";
    import ProjectTab from "./components/ProjectTab/index.svelte";

    export let need_init: boolean;
    export let load_project: boolean;

</script>

{#if need_init}
    <ConfigModal open={need_init} bind:load_project />
{/if}

{#if load_project}
    <ProjectTab />
{/if}
