<script lang="ts">
	import { onMount } from 'svelte';
	import ConfigModal from './components/ConfigModal.svelte';
	import ProjectTab from './components/ProjectTab/index.svelte';
	import { invoke } from '@tauri-apps/api';
	import { sourcePath } from '@/store';
	import { toast } from '@zerodevx/svelte-toast';
	import { toastTheme } from '@/consts';
	import type { Config, SuccessResponse } from '@/types/public';

	let need_init = false;
	let load_project = false;
	let config: Config | undefined;

	$: if ($sourcePath) {
		loadProject();
	}

	function loadProject() {
		invoke<SuccessResponse<Config>>('load_project_config', { sourcePath: $sourcePath })
			.then((res) => {
				toast.push(res.message, toastTheme.success);
				config = res.data;
				need_init = false;
				load_project = true;
			})
			.catch((e) => {
				toast.push(JSON.stringify(e), toastTheme.error);
				config = undefined;
				load_project = false;
				need_init = true;
			});
	}

	// onMount(() => {
	// 	invoke<SuccessResponse<Config>>('load_project', { sourcePath: $sourcePath })
	// 		.then((res) => {
	// 			toast.push(res.message, toastTheme.success);
	// 			config = res.data;
	// 			load_project = true;
	// 		})
	// 		.catch((e) => {
	// 			toast.push(JSON.stringify(e), toastTheme.error);
	// 			need_init = true;
	// 		});
	// });
</script>

{#if need_init}
	<ConfigModal open={need_init} bind:load_project loadProject={loadProject} />
{/if}

{#if load_project}
	<ProjectTab bind:config loadProject={loadProject} />
{/if}
