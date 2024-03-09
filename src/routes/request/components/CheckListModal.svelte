<script lang="ts">
	import Dialog, { Title, Content, Header } from '@smui/dialog';
	import type { RequestString } from '@/types/public';
	import { tweened } from 'svelte/motion';
	import { cubicOut } from 'svelte/easing';
	import Checkbox from '@smui/checkbox';
	import Button from '@smui/button';
	import { invoke } from '@tauri-apps/api';
	import { toast } from '@zerodevx/svelte-toast';
	import { toastTheme } from '@/consts';
	import { PreviewModalContent, PreviewModalOpen, sourcePath } from '@/store';
	import Tooltip, { Wrapper } from '@smui/tooltip';

	export let open = false;

	export let checkList: RequestString[] = [];

	function onClose() {
		checkList = [];
		open = false;
	}

	async function onConfirm() {
		const confirmed = await confirm('操作将重新生成文件，请确保本地代码已经保存！');

		if (!confirmed) return;

		for (let task of checkList) {
			if (!task.checked) continue;
			invoke('write_request_to_file', {
				path: task.full_path,
				content: task.content,
				sourcePath: $sourcePath
			}).catch((e) => {
				toast.push(JSON.stringify(e), toastTheme.error);
			});
		}

		toast.push('生成成功', toastTheme.success);
		open = false;
		checkList = [];
	}

	function preview(content: string) {
		$PreviewModalOpen = true;
		$PreviewModalContent = content;
	}
</script>

<Dialog
	bind:open
	fullscreen
	aria-labelledby="simple-title"
	aria-describedby="simple-content"
	on:SMUIDialog:closed={onClose}
>
	<Header>
		<Title id="fullscreen-title">日志</Title>
		<button style="background:#fff;" on:click={() => (open = false)}>&#x2715;</button>
	</Header>
	<Content id="fullscreen-content">
		<div>请勾选想要生成 ts 类型的接口：</div>
		<div
			style="height:300px;overflow-y:auto;display:flex;flex-direction:column;gap:12px;margin-top:16px"
		>
			{#each checkList as log}
				<div style="display:flex; align-items:start;justify-content:space-between">
					<div style="display:flex; gap:6px">
						<Checkbox checked={log.checked} />
						<div>
							<div style="font-weight:bold">{log.name}</div>
							<div>{log.full_path.replaceAll('\\', '/')}</div>
						</div>
					</div>
					<button
						class="flex-inline items-end"
						style="margin-left:1em;margin-top:8px;"
						on:click={() => preview(log.content)}
					>
						<Wrapper>
							<img class="icon" src="/preview.svg" alt="查看生成的代码" />
							<Tooltip>查看生成的代码</Tooltip>
						</Wrapper>
					</button>
				</div>
			{/each}
		</div>

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
