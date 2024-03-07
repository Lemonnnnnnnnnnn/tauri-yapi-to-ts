<script lang="ts">
	import { toastTheme } from '@/consts';
	import type { SuccessResponse, TypesTree } from '@/types/public';
	import { request } from '@/utils';
	import { toast } from '@zerodevx/svelte-toast';
	import { onMount } from 'svelte';
	import Node from './Node.svelte';
	import Button from '@smui/button';
	import { confirm } from '@tauri-apps/api/dialog';
	import Textfield from '@smui/textfield';
	import { invoke } from '@tauri-apps/api';
	import { sourcePath } from '@/store';

	let full_list: TypesTree[] = [];
	let list: TypesTree[] = [];
	let searchKey = '';

	onMount(() => {
		get_data();
	});

	function get_data() {
		invoke<SuccessResponse<TypesTree[]>>('load_file_tree', { sourcePath: $sourcePath })
			.then((res) => {
				full_list = sort(res.data);
				list = sort(res.data);
				toast.push(JSON.stringify(res.message), toastTheme.success);
			})
			.catch((e) => {
				toast.push(JSON.stringify(e), toastTheme.error);
			});
	}

	function filter_data() {
		if (searchKey === '') {
			list = full_list;
			return;
		}

		list = do_filter(full_list);

		function do_filter(list: TypesTree[]) {
			return list.filter((item) => {
				if (item.name.includes(searchKey)) {
					return true;
				} else {
					if (item.children.length) {
						item.children = do_filter(item.children);

						if (item.children.length) {
							return true;
						}
					}

					return false;
				}
			});
		}
	}

	async function update_service() {
		const confirmed = await confirm('操作将重新生成ts文件，请确保本地代码已经保存！');

		if (!confirmed) return;

		const full_path = list.map((item) => item.full_path);

		request('update_request', { full_path })
			.then((res: SuccessResponse<null>) => {
				toast.push(JSON.stringify(res.message), toastTheme.success);
			})
			.catch((e) => {
				toast.push(JSON.stringify(e), toastTheme.error);
			});
	}

	function sort(list: TypesTree[]) {
		list.forEach((item) => {
			item.children.sort((a, b) => {
				if (a.children.length) {
					return -1;
				}
				return 0;
			});
			if (item.children.length) {
				sort(item.children);
			}
		});

		return list;
	}
</script>

<main style="height:95%;display:flex ; flex-direction:column">
	<div>
		<div class="flex justify-between items-center">
			<div class="header">对应的接口树：</div>
			<Button on:click={update_service}>更新所有请求</Button>
		</div>
		<div class="flex items-center" style="margin-top:10px;margin-bottom:10px">
			<Textfield
				style="flex:1"
				variant="outlined"
				bind:value={searchKey}
				label="搜索"
				on:input={filter_data}
			></Textfield>
		</div>
	</div>

	<div style="flex:1; overflow:auto;display:flex;flex-direction:column;gap:12px">
		{#each list as item}
			<Node {...item} expanded={false} />
		{/each}
	</div>
</main>

<style>
	main {
		margin-left: 10px;
		max-height: 100%;
		max-width: 100%;
		overflow: hidden;
	}

	.header {
		font-size: 22px;
		font-weight: bold;
	}
</style>
