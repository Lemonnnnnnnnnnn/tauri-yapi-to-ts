<script lang="ts">
	import type { InterfaceType, SuccessResponse } from '@/types/public';
	import { startTask } from '@/utils';
	import { PreviewModalContent, PreviewModalOpen, sourcePath } from '@/store';
	import Tooltip, { Wrapper } from '@smui/tooltip';
	import { invoke } from '@tauri-apps/api';
	import type { InterfaceDataItem } from '@/types/yapi';
	import { toast } from '@zerodevx/svelte-toast';
	import { toastTheme } from '@/consts';

	export let data: InterfaceType;
	export let token: string;

	async function update_interface(id: string) {
		await invoke('add_interface_task', {
			data: {
				token,
				source_path: $sourcePath,
				interface_id: Number(id)
			}
		});
		startTask();
	}

	async function preview(interface_id: number) {
		toast.push('正在获取接口信息...', toastTheme.success);
		await invoke<SuccessResponse<InterfaceDataItem>>('get_interface_detail', {
			data: {
				token,
				source_path: $sourcePath,
				interface_id
			}
		}).then((res) => {
			$PreviewModalOpen = true;
			$PreviewModalContent = res.data.ts;
			toast.push(JSON.stringify(res.message), toastTheme.success);
		});
	}
</script>

<div class="flex justify-between items-center" style="margin-bottom: 10px">
	<div>
		{data.name}
	</div>
	<div style="display:flex;align-items:center">
		<div
			style="margin-left: 10px; cursor:pointer"
			on:click|stopPropagation={() => update_interface(data.id)}
			on:keypress|stopPropagation={() => update_interface(data.id)}
			role="button"
			tabindex="0"
		>
			<Wrapper>
				<img class="icon" src="/update.svg" alt="更新接口" />
				<Tooltip>更新接口</Tooltip>
			</Wrapper>
		</div>
		<button
			style="margin-left:1em;margin-bottom:0.2em"
			on:click={() => preview(Number(data.id))}
		>
			<Wrapper>
				<img class="icon" src="/preview.svg" alt="查看生成的代码" />
				<Tooltip>查看生成的代码</Tooltip>
			</Wrapper>
		</button>
	</div>
</div>

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
