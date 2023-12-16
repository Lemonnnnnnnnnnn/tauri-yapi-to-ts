<script lang="ts">
	import { Accordion, AccordionItem, InlineLoading } from 'carbon-components-svelte';
	import type { CategoryType, QueueStatus, SuccessResponse } from '@/types/public';
	import Interface from './Interface.svelte';
	import { request } from '@/utils';
	import { toast } from '@zerodevx/svelte-toast';
	import { toastTheme } from '@/consts';
	import { processingModalTotal, processingModalOpen, runningTask } from '@/store';
	import { confirm } from '@tauri-apps/api/dialog';

	export let data: CategoryType;
	export let token: string;

	$: interfaces = data.interfaces;

	async function update_category(id: string, name: string, is_full_update: boolean) {
		if ($runningTask) {
			toast.push('正在执行任务...请稍等', toastTheme.error);
			return;
		}

		const confirmed = await confirm('操作将重新生成ts文件，是否确定？');

		if (!confirmed) return;

		let categories = [
			{
				id,
				name
			}
		];

		toast.push('正在添加任务...');
		request('update_categories', { categories, token, is_full_update })
			// @ts-expect-error
			.then((res: SuccessResponse<number>) => {
				if (res.data === 0) {
					toast.push('无待执行的任务');
				} else {
					toast.push(res.message, toastTheme.success);
					processingModalOpen.update(() => true);
					processingModalTotal.update(() => res.data);
					runningTask.update(() => true);
				}
			})
			.catch((e) => {
				toast.push(JSON.stringify(e), toastTheme.error);
			});
	}
</script>

<Accordion align="start">
	<AccordionItem>
		<svelte:fragment slot="title">
			<div class="flex justify-between items-center">
				<div>
					{data.name}
				</div>
				<div class="flex items-center" style="height: 30px;">
					<div
						style="margin-left: 10px;margin-right : 10px; cursor:pointer"
						on:click|stopPropagation={() => update_category(data.id, data.name, true)}
						on:keypress|stopPropagation={() => update_category(data.id, data.name, true)}
						role="button"
						tabindex="0"
					>
						<img class="icon" src="/full_update.svg" alt="全量更新分类下的接口" />
					</div>

					<div
						style="margin-left: 10px;margin-right : 10px; cursor:pointer"
						on:click|stopPropagation={() => update_category(data.id, data.name, false)}
						on:keypress|stopPropagation={() => update_category(data.id, data.name, false)}
						role="button"
						tabindex="0"
					>
						<img class="icon" src="/update.svg" alt="增量更新分类下的接口" />
					</div>
				</div>
			</div>
		</svelte:fragment>

		{#each interfaces as data}
			{#if data.name}
				<Interface {data} {token} />
			{/if}
		{/each}
	</AccordionItem>
</Accordion>

<style>
	.icon {
		width: 20px;
		height: 20px;
	}
</style>
