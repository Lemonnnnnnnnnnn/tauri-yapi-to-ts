<script lang="ts">
	import Dialog, { Header, Content, Title } from '@smui/dialog';
	import Textfield from '@smui/textfield';
	import { loadConfig, startTask } from '@/utils';
	import { toast } from '@zerodevx/svelte-toast';
	import { toastTheme } from '@/consts';
	import type { CategoryType, SuccessResponse } from '@/types/public';
	import { sourcePath } from '@/store';
	import Button, { Label } from '@smui/button';
	import { invoke } from '@tauri-apps/api';
	import type { CategoryDataList, CategoryMenuList, InterfaceDataItem } from '@/types/yapi';

	let initForm = {
		interface_id: ''
	};

	export let open: boolean;
	export let token: string;
	export let categories: CategoryType[];

	let form = initForm;

	async function on_submit() {
		if (!form.interface_id) {
			toast.push('请输入Yapi接口ID', toastTheme.error);
			return;
		}

		try {
			toast.push('正在获取接口信息...', toastTheme.success);
			const interfaceDataItem = await invoke<
				SuccessResponse<{ interface_data: InterfaceDataItem; ts: string }>
			>('get_interface_detail', {
				data: {
					token,
					source_path: $sourcePath,
					interface_id: Number(form.interface_id)
				}
			}).then((res) => res.data.interface_data);

			if (isCategoryExist(interfaceDataItem.catid, categories)) {
				toast.push(`接口所在分类已存在`, toastTheme.success);
			} else {
				toast.push(`接口所在分类不存在，更新分类元数据...`, toastTheme.success);
				await invoke<SuccessResponse<CategoryMenuList>>('get_yapi_project_cat_menu', {
					token,
					sourcePath: $sourcePath,
					projectId: Number(interfaceDataItem.project_id)
				});
			}

			toast.push(`更新分类下的接口元数据...`, toastTheme.success);
			await invoke<SuccessResponse<CategoryDataList>>('get_cat_interface_list', {
				token,
				sourcePath: $sourcePath,
				catId: interfaceDataItem.catid
			});
			loadConfig($sourcePath);

			await invoke('add_interface_task', {
				data: {
					token,
					source_path: $sourcePath,
					interface_id: Number(form.interface_id)
				}
			});

			startTask();
			open = false;
		} catch (e) {
			toast.push(JSON.stringify(e), toastTheme.error);
		}
	}

	function isCategoryExist(catId: number, catList: CategoryType[]) {
		return !!catList.find((c) => Number(c.id) === catId);
	}
</script>

<Dialog bind:open fullscreen aria-labelledby="simple-title" aria-describedby="simple-content">
	<Header>
		<Title id="fullscreen-title">新增接口</Title>
		<button style="background:#fff;" on:click={() => (open = false)}>&#x2715;</button>
	</Header>
	<Content style="display:flex; flex-direction:column; gap:12px;padding-top:12px;">
		<Textfield variant="outlined" bind:value={form.interface_id} label="Yapi接口id"></Textfield>
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
