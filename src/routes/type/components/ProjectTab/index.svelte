<script lang="ts">
	import Tab, { Label } from '@smui/tab';
	import TabBar from '@smui/tab-bar';
	import Button from '@smui/button';
	import type { Config } from '@/types/public';
	import { onMount } from 'svelte';
	import AddProjectModal from './AddProjectModal.svelte';
	import ProcessingModal from './ProcessingModal.svelte';
	import Project from './Project.svelte';
	import { config } from '@/store';

	export let loadProject: () => void;
	let openAddModal = false;

	$: project_list = $config?.project_list || [];

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
</script>

<ProcessingModal />
<AddProjectModal {loadProject} bind:open={openAddModal} {project_list} />

<div style="height:95vh; display:flex; flex-direction:column; overflow:auto">
	<div>
		<div>
			<Button on:click={() => (openAddModal = true)}>添加新项目</Button>
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
		<Project
			token={active.token}
			project_id={active.project_id}
			full_category_list={active.categories}
		/>
	{/if}
</div>
