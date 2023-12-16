<script lang="ts">
	import { Button, Search, Tab, TabContent, Tabs } from 'carbon-components-svelte';
	import type { Config, ProjectList, QueueStatus, SuccessResponse } from '@/types/public';
	import { onMount } from 'svelte';
	import Category from './Category.svelte';
	import { request } from '@/utils';
	import { toast } from '@zerodevx/svelte-toast';
	import { toastTheme } from '@/consts';
	import AddProjectModal from './AddProjectModal.svelte';
	import { listen } from '@tauri-apps/api/event';
	import ProcessingModal from '@/components/ProcessingModal.svelte';
	import { processingModalOpen, processingModalTotal, runningTask } from '@/store';
	import { confirm } from '@tauri-apps/api/dialog';
	import { slide } from 'svelte/transition';

	let config: Config | undefined;
	let openAddModal = false;
	let ready = false;
	let searchKey = '';
	let banInitModal = false;

	onMount(() => {
		getConfig();
	});

	listen('task_completed', (_) => {
		toast.push('任务已完成');
		getConfig();
	});

	function getConfig() {
		request('get_config', { key: searchKey })
			// @ts-expect-error
			.then((res: SuccessResponse<Config>) => {
				config = res.data;
			})
			.catch((e) => {
				toast.push(JSON.stringify(e), toastTheme.error);
			})
			.finally(() => {
				ready = true;
			});
	}

	async function fetchProjects(is_full_update: boolean, project_id?: string) {
		if ($runningTask) {
			toast.push('正在执行任务...请稍等', toastTheme.error);
			return;
		}

		const confirmed = await confirm('操作将重新生成ts文件，是否确定？');

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

	function search() {
		banInitModal = true;
		getConfig();
	}

	$: project_list = config?.project_list || [];

	$: {
		if (!banInitModal && ready && project_list.length == 0) {
			openAddModal = true;
		}
	}
</script>

<ProcessingModal />
<AddProjectModal bind:open={openAddModal} bind:banInitModal />

<div>
	<Button kind="ghost" on:click={() => (openAddModal = true)}>添加新项目</Button>
	<Button kind="ghost" on:click={() => fetchProjects(true)}>全量更新所有项目</Button>
	<Button kind="ghost" on:click={() => fetchProjects(false)}>增量更新所有项目</Button>
</div>
<div class="flex items-center">
	<Search bind:value={searchKey} on:blur={() => (banInitModal = false)} />
	<Button kind="secondary" on:click={search}>搜索</Button>
</div>
<Tabs>
	{#each project_list as project}
		<Tab label={project.project_id} />
	{/each}

	<svelte:fragment slot="content">
		{#each project_list as project}
			<TabContent>
				<div transition:slide={{ duration: 300 }}>
					<div style="margin-bottom:10px">
						<Button on:click={() => fetchProjects(true, project.project_id)}
							>全量更新当前项目</Button
						>
						<Button on:click={() => fetchProjects(false, project.project_id)}
							>增量更新当前项目</Button
						>
					</div>
					{#each project.categories as category}
						<Category data={category} token={project.token} />
					{/each}
				</div>
			</TabContent>
		{/each}
	</svelte:fragment>
</Tabs>
