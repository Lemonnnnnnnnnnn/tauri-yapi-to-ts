<script lang="ts">
	import { listen } from '@tauri-apps/api/event';
	import Dialog, { Title, Content, Header } from '@smui/dialog';
	import { onDestroy, onMount } from 'svelte';
	import type { SuccessResponse } from '@/types/public';
	import { toast } from '@zerodevx/svelte-toast';
	import { toastTheme } from '@/consts';
	import { tweened } from 'svelte/motion';
	import { cubicOut } from 'svelte/easing';
	import LinearProgress from '@smui/linear-progress';
	import type { QueueLog, ResolvedInterface } from '@/types/yapi';
	import Checkbox from '@smui/checkbox';
	import Button from '@smui/button';
	import { invoke } from '@tauri-apps/api';
	import {
		PreviewModalContent,
		PreviewModalOpen,
		processingModalOpen,
		processingModalTotal,
		sourcePath
	} from '@/store';
	import Tooltip, { Wrapper } from '@smui/tooltip';

	const progress = tweened(0, {
		duration: 400,
		easing: cubicOut
	});

	let log_area: HTMLDivElement;

	let checkList: (ResolvedInterface & { checked: boolean })[] = [];

	let unlistenLog: () => void;

	onMount(async () => {
		unlistenLog = await listen<QueueLog>('queue_log', (event) => {
			if (event.payload.is_success) {
				checkList.push({ ...event.payload.resolved_interface, checked: true });
				checkList = checkList;
				log_area.scrollTop = log_area.scrollHeight;
				progress.set(checkList.length / $processingModalTotal);
			} else {
				toast.push(JSON.stringify(event.payload.msg), toastTheme.error);
			}
		});
	});

	onDestroy(() => {
		unlistenLog();
	});

	function onClose() {
		const over = checkList.length === $processingModalTotal;

		if (!over) {
			invoke<SuccessResponse<null>>('cancel_task')
				.then((res) => {
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
	}

	async function onConfirm() {
		const confirmed = await confirm('操作将重新生成文件，请确保本地代码已经保存！');

		if (!confirmed) return;

		for (let task of checkList) {
			if (!task.checked) continue;
			invoke('write_to_file', {
				path: task.interface.path,
				content: task.ts_string,
				sourcePath: $sourcePath
			}).catch((e) => {
				toast.push(JSON.stringify(e), toastTheme.error);
			});
		}

		toast.push('生成成功', toastTheme.success);
		$processingModalOpen = false;
	}

	function preview(content: string) {
		$PreviewModalOpen = true;
		$PreviewModalContent = content;
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
			style="height:300px;overflow-y:auto;display:flex;flex-direction:column;gap:12px"
		>
			{#each checkList as log}
				<div style="display:flex; align-items:start;justify-content:space-between">
					<div style="display:flex; gap:6px; align-items:center">
						<Checkbox bind:checked={log.checked} />
						<span>{log.interface.title}</span>
						<span>{log.interface.path}</span>
					</div>
					<button
						class="flex-inline items-end"
						style="margin-left:1em;margin-top:8px;"
						on:click={() => preview(log.ts_string)}
					>
						<Wrapper>
							<img class="icon" src="/preview.svg" alt="查看生成的代码" />
							<Tooltip>查看生成的代码</Tooltip>
						</Wrapper>
					</button>
				</div>
			{/each}
		</div>
		<LinearProgress progress={$progress} />
		<div style="margin:32px 16px 0 16px;">
			<Button
				style="display:flex; justify-content:center;width:100%;"
				variant="raised"
				on:click={onConfirm}>确定</Button
			>
		</div>
	</Content>
</Dialog>

<style>
	button {
		background: #fff;
		cursor: pointer;
		border: none;
		padding: 0;
	}
	.icon {
		width: 20px;
		height: 20px;
	}
</style>
