<script lang="ts">
	import ConfigModal from './components/ConfigModal.svelte';
	import ProjectTab from './components/ProjectTab/index.svelte';
	import { sourcePath  } from '@/store';
	import { loadConfig } from '@/utils';

	let need_init = false;
	let load_project = false;

	$: if ($sourcePath) {
		loadProject();
	}

	function loadProject() {
		loadConfig($sourcePath)
			.then(() => {
				need_init = false;
				load_project = true;
			})
			.catch(() => {
				load_project = false;
				need_init = true;
			});
	}
	
</script>

{#if need_init}
	<ConfigModal open={need_init} bind:load_project {loadProject} />
{/if}

{#if load_project}
	<ProjectTab {loadProject} />
{/if}
