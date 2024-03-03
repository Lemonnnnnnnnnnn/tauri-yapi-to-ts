<script lang="ts">
	import { listen } from '@tauri-apps/api/event';
	import Dialog, { Title, Content, Header } from '@smui/dialog';
	import { onDestroy, onMount } from 'svelte';
	import { runningTask, processingModalOpen, processingModalTotal } from '../../../../lib/store';
	import { request } from '@/utils';
	import type { SuccessResponse } from '@/types/public';
	import { toast } from '@zerodevx/svelte-toast';
	import { toastTheme } from '@/consts';
	import { tweened } from 'svelte/motion';
	import { cubicOut } from 'svelte/easing';
	import LinearProgress from '@smui/linear-progress';
	import type { QueueLog, ResolvedInterface } from '@/types/yapi';
	import Checkbox from '@smui/checkbox';
	import Button from '@smui/button';

	const progress = tweened(0, {
		duration: 400,
		easing: cubicOut
	});

	let log_area: HTMLDivElement;

	let checkList: (ResolvedInterface & { checked: boolean })[] = [];

	let unlistenLog: () => void;

	onMount(async () => {
		unlistenLog = await listen<QueueLog>('queue_log', (event) => {
			checkList.push({ ...event.payload.resolved_interface, checked: true });
			checkList = checkList;
			log_area.scrollTop = log_area.scrollHeight;
			progress.set(checkList.length / $processingModalTotal);
		});
	});

	onDestroy(() => {
		unlistenLog();
	});

	function onClose() {
		const over = checkList.length === $processingModalTotal;

		if (!over) {
			request('cancel_task')
				.then((res: SuccessResponse<null>) => {
					toast.push(JSON.stringify(res.message), toastTheme.success);
					$processingModalOpen = false;
				})
				.catch((e) => {
					toast.push(JSON.stringify(e), toastTheme.error);
				});
		}

		checkList = [];
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
		<button style="background:#fff;" on:click={() => ($processingModalOpen = false)}
			>&#x2715;</button
		>
	</Header>
	<Content id="fullscreen-content">
		<div>请勾选想要生成 ts 类型的接口：</div>
		<div
			bind:this={log_area}
			style="max-height:300px;overflow-y:auto;display:flex;flex-direction:column;gap:12px"
		>
			{#each checkList as log}
				<div style="display:flex; gap:6px; align-items:center">
					<Checkbox checked={log.checked} />
					<span>{log.interface.title}</span>
					<span>{log.interface.path}</span>
				</div>
			{/each}
		</div>
		<div style="width:100%;margin-top:12px;margin-bottom:12px">
			<Button style="display:flex; justify-content:end">确定</Button>
		</div>
		<LinearProgress progress={$progress} />
	</Content>
</Dialog>

<style>
</style>
