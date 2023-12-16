<script lang="ts">
	import { toast } from '@zerodevx/svelte-toast';
	import type { InterfaceType, QueueStatus, SuccessResponse } from '@/types/public';
	import { request } from '@/utils';
	import { toastTheme } from '@/consts';
	import { InlineLoading } from 'carbon-components-svelte';
	import { processingModalTotal, processingModalOpen, runningTask } from '@/store';
	import { confirm } from '@tauri-apps/api/dialog';

	export let data: InterfaceType;
	export let token: string;

	async function update_interface(id: string) {
		if ($runningTask) {
			toast.push('正在执行任务...请稍等', toastTheme.error);
			return;
		}
		const confirmed = await confirm('操作将重新生成ts文件，是否确定？');

		if (!confirmed) return;

		let interfaces = [
			{
				id
			}
		];

		toast.push('正在添加任务...');
		request('update_interface', { interfaces, token })
			// @ts-expect-error
			.then((res: SuccessResponse<number>) => {
				toast.push(res.message, toastTheme.success);
				processingModalOpen.update(() => true);
				processingModalTotal.update(() => res.data);
				runningTask.update(() => true);
			})
			.catch((e) => {
				toast.push(JSON.stringify(e), toastTheme.error);
			});
	}
</script>

<div class="flex justify-between items-center" style="margin-bottom: 10px">
	<div>
		{data.name}
	</div>
	<div>
		<div
			style="margin-left: 10px; cursor:pointer"
			on:click|stopPropagation={() => update_interface(data.id)}
			on:keypress|stopPropagation={() => update_interface(data.id)}
			role="button"
			tabindex="0"
		>
			<img class="icon" src="/full_update.svg" alt="更新接口" />
		</div>
	</div>
</div>

<style>
	.icon {
		width: 20px;
		height: 20px;
	}
</style>
