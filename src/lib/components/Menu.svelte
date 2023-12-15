<script lang="ts">
    import TypeModule from "../modules/TypeModule/index.svelte";
    import ServiceModule from "../modules/ServiceModule/index.svelte";
    import ConfigModule from "../modules/ConfigModule/index.svelte";

    export let need_init: boolean;
    export let load_project: boolean;

    let menuList = [
        {
            key: "1",
            label: "Type",
        },
        {
            key: "2",
            label: "Service",
        },
        {
            key: "3",
            label: "Config",
        },
    ];

    let defaultActiveMenu = menuList[0].key;
    let activeMenu = defaultActiveMenu;

    function selectMenu(key: string) {
        activeMenu = key;
    }
</script>

<div class="wrapper">
    <aside>
        {#each menuList as item}
            <div
                on:click={() => selectMenu(item.key)}
                on:keypress={() => selectMenu(item.key)}
                role="button"
                tabindex={Number(item.key)}
                class="menu-item"
                class:active={activeMenu === item.key}
            >
                {item.label}
            </div>
        {/each}
    </aside>

    <main>
        {#if activeMenu === "1"}
            <TypeModule {need_init} {load_project} />
        {:else if activeMenu === "2"}
            <ServiceModule />
        {:else if activeMenu === "3"}
            <ConfigModule />
        {/if}
    </main>
</div>

<style lang="scss">
    aside {
        width: 150px;
        height: 100vh;
        border-right: 1px solid rgba(0, 0, 0, 0.1);
    }

    main {
        flex: 1;
        padding: 10px;
    }

    .wrapper {
        display: flex;
        height: 100vh;
        width: 100%;
        overflow: auto;
    }

    .menu-item {
        padding: 10px;
        transition: all 0.2s ease-in-out;
        cursor: pointer;
        text-align: center;

        &.active {
            font-weight: bold;
            transform: scale(1.1);
        }
    }

    .menu-item:not(:last-child) {
        border-bottom: 1px solid rgba(0, 0, 0, 0.1);
    }

    /* .icon {
        height: 20px;
        width: 20px;
    } */
</style>
