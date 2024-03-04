<script lang="ts">
	import Tab, { Label } from '@smui/tab';
	import TabBar from '@smui/tab-bar';
	import Button from '@smui/button';
	import type { Config, SuccessResponse } from '@/types/public';
	import { SvelteComponent, onMount } from 'svelte';
	import Category from './Category.svelte';
	import { request } from '@/utils';
	import { toast } from '@zerodevx/svelte-toast';
	import { toastTheme } from '@/consts';
	import AddProjectModal from './AddProjectModal.svelte';
	import { listen } from '@tauri-apps/api/event';
	import ProcessingModal from './ProcessingModal.svelte';
	import { processingModalOpen, processingModalTotal, runningTask } from '@/store';
	import { confirm } from '@tauri-apps/api/dialog';
	import { slide } from 'svelte/transition';
	import Textfield from '@smui/textfield';
	import Accordion from '@smui-extra/accordion';

	export let config: Config | undefined;
	let openAddModal = false;
	let searchKey = '';
	$: project_list = config?.project_list || [];

	let active : NonNullable<Config['project_list']>[number] = {
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
		// getConfig();
		active = project_list[0];
	});

	listen('task_completed', (_) => {
		toast.push('任务已完成');
	});

	async function fetchProjects(is_full_update: boolean, project_id?: string) {
		if ($runningTask) {
			toast.push('正在执行任务...请稍等', toastTheme.error);
			return;
		}

		const confirmed = await confirm('操作将重新生成ts文件，请确保本地代码已经保存！');

		if (!confirmed) return;

		let project_list = config?.project_list || [];

		let projects = project_list.map((project) => ({
			id: project.project_id,
			token: project.token
		}));

		if (project_id) {
			projects = projects.filter((project) => project.id === project_id);
		}
		toast.push('正在添加任务...');
		request('update_projects', { projects, is_full_update })
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

	function search() {
		// banInitModal = true;
		// getConfig();
	}
</script>

<ProcessingModal />
<AddProjectModal bind:open={openAddModal} project_list={project_list} />

<div style="height:95vh; display:flex; flex-direction:column; overflow:auto">
	<div>
		<div>
			<Button on:click={() => (openAddModal = true)}>添加新项目</Button>
			<Button on:click={() => fetchProjects(true)}>全量更新所有项目</Button>
			<Button on:click={() => fetchProjects(false)}>增量更新所有项目</Button>
		</div>
		<div class="flex items-center">
			<Textfield variant="outlined" bind:value={searchKey} style="flex:1" label="搜索"></Textfield>

			<Button style="height:56px" color="secondary" variant="raised" on:click={search}>搜索</Button>
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
			<div transition:slide={{ duration: 300 }}>
				<div style="margin-bottom:10px">
					<Button variant="raised" on:click={() => fetchProjects(true, active.project_id)}
						>全量更新当前项目</Button
					>
					<Button variant="raised" on:click={() => fetchProjects(false, active.project_id)}
						>增量更新当前项目</Button
					>
				</div>
			</div>
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
