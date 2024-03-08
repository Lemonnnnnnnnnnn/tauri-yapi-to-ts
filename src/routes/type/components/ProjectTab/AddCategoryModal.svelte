<script lang="ts">
	import Dialog, { Header, Content, Title } from '@smui/dialog';
	import Textfield from '@smui/textfield';
	import { loadConfig, startTask } from '@/utils';
	import { toast } from '@zerodevx/svelte-toast';
	import { toastTheme } from '@/consts';
	import type { SuccessResponse } from '@/types/public';
	import { sourcePath } from '@/store';
	import Button, { Label } from '@smui/button';
	import { invoke } from '@tauri-apps/api';
	import type { CategoryDataList, CategoryMenuList } from '@/types/yapi';

	let initForm = {
		cat_id: ''
	};

	export let open: boolean;
	export let token: string;
	export let projectId: string

	let form = initForm;

	async function on_submit() {
		if (!form.cat_id) {
			toast.push('请输入Yapi分类', toastTheme.error);
			return;
		}

		try {
			toast.push('正在更新分类基本信息...', toastTheme.success);
			await invoke<SuccessResponse<CategoryMenuList>>(
				'get_yapi_project_cat_menu',
				{
					token,
					sourcePath: $sourcePath,
					projectId: Number(projectId),
				}
			);
			
			toast.push(`正在获取分类下的接口...`, toastTheme.success);
			const interfaceList = await invoke<SuccessResponse<CategoryDataList>>(
				'get_cat_interface_list',
				{
					token,
					sourcePath: $sourcePath,
					catId: Number(form.cat_id)
				}
			);

			for await (let i of interfaceList.data.list) {
				await invoke('add_interface_task', {
					data: {
						token,
						source_path: $sourcePath,
						interface_id: i._id
					}
				});
			}
			loadConfig($sourcePath)
			startTask();
			open = false;
		} catch (e) {
			toast.push(JSON.stringify(e), toastTheme.error);
		}
	}
</script>

<Dialog bind:open fullscreen aria-labelledby="simple-title" aria-describedby="simple-content">
	<Header>
		<Title id="fullscreen-title">新增分类</Title>
		<button style="background:#fff;" on:click={() => (open = false)}>&#x2715;</button>
	</Header>
	<Content style="display:flex; flex-direction:column; gap:12px;padding-top:12px;">
		<Textfield variant="outlined" bind:value={form.cat_id} label="Yapi分类id"></Textfield>
	</Content>
	<div style="display:flex; justify-content:flex-end; margin-bottom:12px;">
		<Button on:click={() => (open = false)}>
			<Label>取消</Label>
		</Button>
		<Button on:click={on_submit}>
			<Label>提交</Label>
		</Button>
	</div>
</Dialog>
