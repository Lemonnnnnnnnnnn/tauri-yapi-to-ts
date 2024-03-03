<script lang="ts">
	import Dialog, { Actions, Header, Content, Title } from '@smui/dialog';
	import Textfield from '@smui/textfield';
	import { request, startTask } from '@/utils';
	import { toast } from '@zerodevx/svelte-toast';
	import { toastTheme } from '@/consts';
	import type { SuccessResponse } from '@/types/public';
	import { runningTask, processingModalOpen, processingModalTotal, sourcePath } from '@/store';
	import { confirm } from '@tauri-apps/api/dialog';
	import Button, { Label } from '@smui/button';
	import { invoke } from '@tauri-apps/api';
	import type { CategoryDataList, CategoryMenuList, ProjectBaseInfo } from '@/types/yapi';

	let initForm = {
		token: ''
	};

	export let open: boolean;

	let form = initForm;

	async function on_submit() {
		// if ($runningTask) {
		// 	toast.push('正在执行任务...请稍等', toastTheme.error);
		// 	return;
		// }

		if (!form.token) {
			toast.push('请输入Yapi项目token', toastTheme.error);
			return;
		}

		try {
			toast.push('正在获取项目信息...', toastTheme.success);
			const baseInfo = await invoke<SuccessResponse<ProjectBaseInfo>>(
				'get_yapi_project_base_info',
				{
					token: form.token,
					sourcePath: $sourcePath
				}
			);
			toast.push('正在获取项目分类...', toastTheme.success);
			const menuList = await invoke<SuccessResponse<CategoryMenuList>>(
				'get_yapi_project_cat_menu',
				{
					token: form.token,
					sourcePath: $sourcePath,
					projectId: baseInfo.data._id
				}
			);

			for await (let catMenu of menuList.data) {
				toast.push(`正在获取分类${catMenu.name}下接口...`, toastTheme.success);
				const categoryDataList = await invoke<SuccessResponse<CategoryDataList>>(
					'get_cat_interface_list',
					{
						token: form.token,
						sourcePath: $sourcePath,
						catId: catMenu._id
					}
				);

				for await (let i of categoryDataList.data.list) {
					await invoke('add_interface_task', {
						data: {
							token: form.token,
							source_path: $sourcePath,
							interface_id: i._id
						}
					});
				}
			}

			startTask();
			open=false;
		} catch (e) {
			toast.push(JSON.stringify(e), toastTheme.error);
		}

		// const confirmed = await confirm('操作将生成ts文件，是否确定？');

		// if (!confirmed) return;

		// toast.push('正在添加任务...');
		// request('update_projects', { projects: [form], is_full_update: true })
		// 	.then((res: SuccessResponse<number>) => {
		// 		if (res.data === 0) {
		// 			toast.push('无待执行的任务');
		// 		} else {
		// 			toast.push(res.message, toastTheme.success);
		// 			processingModalOpen.update(() => true);
		// 			processingModalTotal.update(() => res.data);
		// 			runningTask.update(() => true);
		// 		}
		// 		open = false;
		// 		form = initForm;
		// 	})
		// 	.catch((e) => {
		// 		toast.push(JSON.stringify(e), toastTheme.error);
		// 	});
	}
</script>

<Dialog bind:open fullscreen aria-labelledby="simple-title" aria-describedby="simple-content">
	<Header>
		<Title id="fullscreen-title">新增项目</Title>
		<button style="background:#fff;" on:click={() => (open = false)}>&#x2715;</button>
	</Header>
	<Content style="display:flex; flex-direction:column; gap:12px;padding-top:12px;">
		<Textfield variant="outlined" bind:value={form.token} label="Yapi项目token"></Textfield>
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
