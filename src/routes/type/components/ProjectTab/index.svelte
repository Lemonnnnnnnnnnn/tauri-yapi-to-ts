<script lang="ts">
	import Tab, { Label } from '@smui/tab';
	import TabBar from '@smui/tab-bar';
	import Button from '@smui/button';
	import type { Config, SuccessResponse } from '@/types/public';
	import { onMount } from 'svelte';
	import Category from './Category.svelte';
	import { startTask } from '@/utils';
	import { toast } from '@zerodevx/svelte-toast';
	import { toastTheme } from '@/consts';
	import AddProjectModal from './AddProjectModal.svelte';
	import ProcessingModal from './ProcessingModal.svelte';
	import { sourcePath } from '@/store';
	import Textfield from '@smui/textfield';
	import Accordion from '@smui-extra/accordion';
	import { invoke } from '@tauri-apps/api';
	import type { CategoryDataList, CategoryMenuList } from '@/types/yapi';

	export let config: Config | undefined;
	export let loadProject: () => void;
	let openAddModal = false;
	let searchKey = '';

	$: project_list = config?.project_list || [];

	let active: NonNullable<Config['project_list']>[number] = {
		project_id: '',
		categories: [],
		token: ''
	};

	$: {
		if (project_list.length == 0) {
			openAddModal = true;
		}
	}

	onMount(() => {
		active = project_list[0];
	});

	async function fetchProjects(is_full_update: boolean, project_id?: string) {
		const project = project_list.find((p) => p.project_id === project_id);

		try {
			toast.push('正在获取项目分类...', toastTheme.success);
			const categoryList = await invoke<SuccessResponse<CategoryMenuList>>(
				'get_yapi_project_cat_menu',
				{
					token: project?.token,
					sourcePath: $sourcePath,
					projectId: project_id
				}
			);

			for await (let category of categoryList.data) {
				if (!is_full_update) {
					if (project?.categories.find((c) => c.id === String(category._id))) continue;
				}
				toast.push(`正在获取分类${category.name}下接口...`, toastTheme.success);
				const interfaceList = await invoke<SuccessResponse<CategoryDataList>>(
					'get_cat_interface_list',
					{
						token: project?.token,
						sourcePath: $sourcePath,
						catId: category._id
					}
				);

				for await (let i of interfaceList.data.list) {
					await invoke('add_interface_task', {
						data: {
							token: project?.token,
							source_path: $sourcePath,
							interface_id: i._id
						}
					});
				}
			}

			startTask();
		} catch (e) {
			toast.push(JSON.stringify(e), toastTheme.error);
		}
	}

	function search() {
		// banInitModal = true;
		// getConfig();
	}
</script>

<ProcessingModal />
<AddProjectModal {loadProject} bind:open={openAddModal} {project_list} />

<div style="height:95vh; display:flex; flex-direction:column; overflow:auto">
	<div>
		<div>
			<Button on:click={() => (openAddModal = true)}>添加新项目</Button>
			<Button variant="raised" on:click={() => fetchProjects(true, active.project_id)}
				>更新所有分类下的接口</Button
			>
			<Button variant="raised" on:click={() => fetchProjects(false, active.project_id)}
				>更新新增分类下的接口</Button
			>
		</div>
		<div class="flex items-center">
			<Textfield variant="outlined" bind:value={searchKey} style="flex:1" label="搜索"></Textfield>
		</div>
		{#if active?.project_id}
			<TabBar
				style="margin-top:12px;margin-bottom: 12px;"
				tabs={project_list}
				key={(project) => project.project_id}
				let:tab
				bind:active
			>
				<Tab minWidth {tab}>
					<Label>{tab.project_id}</Label>
				</Tab>
			</TabBar>
		{/if}
	</div>

	{#if active?.project_id}
		<Accordion style="flex:1 ; overflow:auto">
			{#each active.categories as category}
				<Category data={category} token={active.token} />
			{/each}
		</Accordion>
	{/if}
</div>
