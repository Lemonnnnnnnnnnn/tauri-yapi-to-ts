<script lang="ts">
	import type { CategoryType, SuccessResponse } from '@/types/public';
	import Accordion from '@smui-extra/accordion';
	import Category from './Category.svelte';
	import Textfield from '@smui/textfield';
	import Button from '@smui/button';
	import { toast } from '@zerodevx/svelte-toast';
	import { sourcePath } from '@/store';
	import { invoke } from '@tauri-apps/api';
	import { toastTheme } from '@/consts';
	import type { CategoryDataList, CategoryMenuList } from '@/types/yapi';
	import { startTask } from '@/utils';

	export let full_category_list: CategoryType[];
	export let token: string;
	export let project_id: string;
	let filtered_category_list = full_category_list;
	let searchKey = '';

	function search() {
		if (searchKey) {
			// filter category and category.interfaces base on searchKey
			let deepCloneFullCategoryList = JSON.parse(JSON.stringify(full_category_list));

			filtered_category_list = deepCloneFullCategoryList.filter((category) => {
				if (category.name.toLocaleLowerCase().includes(searchKey.toLocaleLowerCase())) {
					return true;
				}
				category.interfaces = category.interfaces.filter((i) => {
					if (
						i.name?.toLocaleLowerCase().includes(searchKey.toLocaleLowerCase()) ||
						i.path?.toLocaleLowerCase().includes(searchKey.toLocaleLowerCase())
					) {
						return true;
					}
					return false;
				});
				if (category.interfaces.length > 0) {
					return true;
				}
				return false;
			});
		} else {
			filtered_category_list = full_category_list;
		}
	}

	async function fetchProjects(is_full_update: boolean) {
		try {
			toast.push('正在获取项目分类...', toastTheme.success);
			const categoryList = await invoke<SuccessResponse<CategoryMenuList>>(
				'get_yapi_project_cat_menu',
				{
					token,
					sourcePath: $sourcePath,
					projectId: Number(project_id)
				}
			);
			let num = 0;

			for await (let category of categoryList.data) {
				if (!is_full_update) {
					if (full_category_list.find((c) => c.id === String(category._id))) continue;
				}
				num++;
				toast.push(`正在获取分类${category.name}下接口...`, toastTheme.success);
				const interfaceList = await invoke<SuccessResponse<CategoryDataList>>(
					'get_cat_interface_list',
					{
						token,
						sourcePath: $sourcePath,
						catId: category._id
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
			}

			if (num) {
				startTask();
			} else {
				toast.push('无需更新', toastTheme.success);
			}
		} catch (e) {
			toast.push(JSON.stringify(e), toastTheme.error);
		}
	}
</script>

<Textfield
	style="margin-bottom:16px"
	variant="outlined"
	bind:value={searchKey}
	on:input={search}
	label="搜索"
></Textfield>
<div style="margin-bottom:16px">
	<Button variant="raised" on:click={() => fetchProjects(true)}>更新所有分类下的接口</Button>
	<Button variant="raised" on:click={() => fetchProjects(false)}>更新新增分类下的接口</Button>
</div>
<Accordion style="flex:1 ; overflow:auto">
	{#each filtered_category_list as category}
		<Category data={category} {token} />
	{/each}
</Accordion>
