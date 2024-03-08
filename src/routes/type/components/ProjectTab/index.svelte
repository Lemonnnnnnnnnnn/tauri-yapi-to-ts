<script lang="ts">
	import Tab, { Label } from '@smui/tab';
	import TabBar from '@smui/tab-bar';
	import Button from '@smui/button';
	import type { Config } from '@/types/public';
	import { onMount } from 'svelte';
	import AddProjectModal from './AddProjectModal.svelte';
	import AddCategoryModal from './AddCategoryModal.svelte';
	import AddInterfaceModal from './AddInterfaceModal.svelte';
	import ProcessingModal from './ProcessingModal.svelte';
	import Project from './Project.svelte';
	import { config } from '@/store';

	export let loadProject: () => void;
	let openAddProjectModal = false;
	let openAddCategoryModal = false;
	let openAddInterfaceModal = false;

	let project_list: Config['project_list'] = [];

	let active: NonNullable<Config['project_list']>[number] = {
		project_id: '',
		project_name: '',
		categories: [],
		token: ''
	};

	config.subscribe((value) => {
		project_list = value?.project_list || [];
		if (project_list?.length > 0) {
			active = project_list[0];
			active.categories = active.categories;
		}
	});

	$: {
		if (project_list?.length == 0) {
			openAddProjectModal = true;
		}
	}

	onMount(() => {
		if (project_list && project_list.length > 0) {
			active = project_list[0];
		}
	});
</script>

<ProcessingModal />
<AddProjectModal {loadProject} bind:open={openAddProjectModal} {project_list} />
<AddCategoryModal
	bind:open={openAddCategoryModal}
	token={active.token}
	projectId={active.project_id}
/>
<AddInterfaceModal
	token={active.token}
	categories={active.categories}
	bind:open={openAddInterfaceModal}
/>

<div style="height:95vh; display:flex; flex-direction:column; overflow:auto">
	<div>
		<div>
			<Button on:click={() => (openAddProjectModal = true)}>添加新项目</Button>
			<Button on:click={() => (openAddCategoryModal = true)}>在当前项目下添加新分类</Button>
			<Button on:click={() => (openAddInterfaceModal = true)}>在当前项目下添加新接口</Button>
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
					<Label>{tab.project_name || tab.project_id}</Label>
				</Tab>
			</TabBar>
		{/if}
	</div>
	{#key active.categories}
		{#if active?.project_id}
			<Project
				token={active.token}
				project_id={active.project_id}
				full_category_list={active.categories}
			/>
		{/if}
	{/key}
</div>
