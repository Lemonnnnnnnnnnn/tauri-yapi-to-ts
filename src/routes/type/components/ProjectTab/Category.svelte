<script lang="ts">
	import { Panel, Header, Content } from '@smui-extra/accordion';
	import type { CategoryType, SuccessResponse } from '@/types/public';
	import Interface from './Interface.svelte';
	import { toast } from '@zerodevx/svelte-toast';
	import { toastTheme } from '@/consts';
	import { sourcePath } from '@/store';
	import Tooltip, { Wrapper } from '@smui/tooltip';
	import { invoke } from '@tauri-apps/api';
	import type { CategoryDataList } from '@/types/yapi';
	import { loadConfig, startTask } from '@/utils';

	export let data: CategoryType;
	export let token: string;

	$: interfaces = data.interfaces;

	async function update_category(id: string, name: string, is_full_update: boolean) {
		try {
			toast.push(`正在获取分类${name}下接口...`, toastTheme.success);
			const interfaceList = await invoke<SuccessResponse<CategoryDataList>>(
				'get_cat_interface_list',
				{
					token,
					sourcePath: $sourcePath,
					catId: Number(id)
				}
			);
			loadConfig($sourcePath)

			for await (let i of interfaceList.data.list) {
				if (!is_full_update) {
					let oldCategory = data;
					if (oldCategory.interfaces.find((old_i) => old_i.id === String(i._id))) continue;
				}
				await invoke('add_interface_task', {
					data: {
						token,
						source_path: $sourcePath,
						interface_id: i._id
					}
				});
			}

			startTask();
			
		} catch (e) {
			toast.push(JSON.stringify(e), toastTheme.error);
		}
	}
</script>

<Panel>
	<Header>
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
					<Wrapper>
						<img class="icon" src="/full_update.svg" alt="全量更新分类下的接口" />
						<Tooltip>更新分类下的所有接口</Tooltip>
					</Wrapper>
				</div>

				<div
					style="margin-left: 10px;margin-right : 10px; cursor:pointer"
					on:click|stopPropagation={() => update_category(data.id, data.name, false)}
					on:keypress|stopPropagation={() => update_category(data.id, data.name, false)}
					role="button"
					tabindex="0"
				>
					<Wrapper>
						<img class="icon" src="/update.svg" alt="增量更新分类下的接口" />
						<Tooltip>更新分类下新增的接口</Tooltip>
					</Wrapper>
				</div>
			</div>
		</div>
	</Header>
	<Content>
		{#each interfaces as data}
			{#if data.name}
				<Interface {data} {token} />
			{/if}
		{/each}
	</Content>
</Panel>

<style>
	.icon {
		width: 20px;
		height: 20px;
	}
</style>
