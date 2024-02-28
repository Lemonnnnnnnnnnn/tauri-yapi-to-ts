<script lang="ts">
	import { listen } from '@tauri-apps/api/event';
	import Dialog, { Title, Content, Header } from '@smui/dialog';
	import { onDestroy, onMount } from 'svelte';
	import { runningTask, processingModalOpen, processingModalTotal } from '../store';
	import { request } from '@/utils';
	import type { SuccessResponse } from '@/types/public';
	import { toast } from '@zerodevx/svelte-toast';
	import { toastTheme } from '@/consts';
	import { tweened } from 'svelte/motion';
	import { cubicOut } from 'svelte/easing';
	import LinearProgress from '@smui/linear-progress';

	const progress = tweened(0, {
		duration: 400,
		easing: cubicOut
	});

	let log_area: HTMLDivElement;

	let log_list: {
		msg: string;
		is_success: boolean;
	}[] = [];

	let unlistenLog: () => void;

	onMount(async () => {
		unlistenLog = await listen<{
			msg: string;
			success_number: number;
			is_success: boolean;
		}>('log', (event) => {
			log_list.push({
				msg: event.payload.msg,
				is_success: event.payload.is_success
			});
			log_list = log_list;
			log_area.scrollTop = log_area.scrollHeight;
			progress.set(event.payload.success_number / $processingModalTotal);
		});
	});

	onDestroy(() => {
		unlistenLog();
	});

	function onClose() {
		const over = log_list.length === $processingModalTotal;

		if (!over) {
			request('pause')
				// @ts-expect-error
				.then((res: SuccessResponse<null>) => {
					toast.push(JSON.stringify(res.message), toastTheme.success);
					$processingModalOpen = false;
				})
				.catch((e) => {
					toast.push(JSON.stringify(e), toastTheme.error);
				});
		}

		log_list = [];
		$processingModalTotal = 0;
		progress.set(0);
		runningTask.update(() => false);
	}
</script>

<Dialog
	bind:open={$processingModalOpen}
	fullscreen
	aria-labelledby="simple-title"
	aria-describedby="simple-content"
	on:SMUIDialog:closed={onClose}
>
	<Header>
		<Title id="fullscreen-title">日志</Title>
		<button style="background:#fff;" on:click={() => ($processingModalOpen = false)}>&#x2715;</button>
	</Header>
	<Content id="fullscreen-content">
		<div bind:this={log_area} style="max-height:300px;overflow-y:auto;">
			{#each log_list as log}
				{#if log.is_success}
					<p>{log.msg}</p>
				{:else}
					<p style="color:crimson">{log.msg}</p>
				{/if}
			{/each}
		</div>
		<LinearProgress progress={$progress} />
	</Content>
</Dialog>

<style>
</style>
