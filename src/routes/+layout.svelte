<script lang="ts">
	import './global.css';
	import Sidebar from '@/components/Sidebar.svelte';
	import Prompt from '@/components/Prompt.svelte';
	import { onDestroy, onMount } from 'svelte';
	import { invoke } from '@tauri-apps/api';
	import { goto } from '$app/navigation';
	import { SvelteToast, toast } from '@zerodevx/svelte-toast';
	import { toastTheme } from '@/consts';
	import { listen } from '@tauri-apps/api/event';
	import { sourcePath } from '@/store';
	import type { SuccessResponse } from '@/types/public';

	let unlistenLoadProject: () => void;
	let unlistenLoadProjectError: () => void;
	let unlistenNotification: () => void;
	let existProject = false;

	onMount(async () => {
		unlistenLoadProject = await listen<string>('load_project', (event) => {
			$sourcePath = event.payload;
			existProject = true;

			if ($sourcePath) {
				goto('/type');
			}
		});

		unlistenLoadProjectError = await listen<string>('load_project_error', (event) => {
			toast.push(event.payload, toastTheme.error);
			// todo: 请查看日志获取更多信息，日志地址xxx
		});

		unlistenNotification = await listen<{ message: string; desc: 'Success' | 'Error' }>(
			'notification',
			(event) => {
				toast.push(event.payload.message, toastTheme[event.payload.desc.toLowerCase()]);
			}
		);

		setTimeout(() => {
			invoke<SuccessResponse<null>>('load_latest_project').catch((e) => {});
		}, 0);
	});

	onDestroy(() => {
		unlistenLoadProject();
		unlistenLoadProjectError();
		unlistenNotification();
	});
</script>

<div class="app">
	<SvelteToast options={{}} />
	{#if existProject}
		<Sidebar />
		<main>
			<slot />
		</main>
	{:else}
		<Prompt />
	{/if}
</div>

<style>
	.app {
		display: flex;
		flex-direction: row;
		height: 100vh;
	}

	main {
		flex: 1;
	}
</style>
