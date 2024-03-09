<script lang="ts">
	import { onMount } from 'svelte';
	import ConfigModal from './components/ConfigModal.svelte';
	import TypesTree from './components/TypesTree.svelte';
	import { config } from '@/store';

	let need_init = false;
	let load_types = false;

	onMount(() => {
		if (!$config.request_path) {
			need_init = true;
		} else {
			load_types = true;
		}
	});
</script>

{#if need_init}
	<ConfigModal bind:open={need_init} bind:load_types />
{/if}

{#if load_types}
	<TypesTree />
{/if}
