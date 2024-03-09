<script lang="ts">
	import { toastTheme } from '@/consts';
	import type { RequestString, SuccessResponse, TypesTree } from '@/types/public';
	import { toast } from '@zerodevx/svelte-toast';
	import { onMount } from 'svelte';
	import Node from './Node.svelte';
	import Button from '@smui/button';
	import Textfield from '@smui/textfield';
	import { invoke } from '@tauri-apps/api';
	import { config, sourcePath } from '@/store';
	import CheckListModal from './CheckListModal.svelte';

	let full_list: TypesTree[] = [];
	let filtered_list: TypesTree[] = [];
	let over_list: RequestString[] = [];
	let searchKey = '';
	let check_list_modal_open = false;

	onMount(() => {
		get_data();
	});

	function get_data() {
		invoke<SuccessResponse<TypesTree[]>>('load_file_tree', { sourcePath: $sourcePath })
			.then((res) => {
				full_list = sort(res.data);
				filtered_list = sort(res.data);
				toast.push(JSON.stringify(res.message), toastTheme.success);
			})
			.catch((e) => {
				toast.push(JSON.stringify(e), toastTheme.error);
			});
	}

	function filter_data() {
		if (searchKey === '') {
			filtered_list = full_list;
			return;
		}

		filtered_list = do_filter(full_list);

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

	function sort(list: TypesTree[]) {
		list.sort((a, b) => {
			if (a.children.length) {
				return 1;
			}
			return -1;
		});
		list.forEach((item) => {
			if (item.children.length) {
				sort(item.children);
			}
		});

		return list;
	}

	async function update_request(node?: TypesTree) {
		if (node) {
			await fetch(node.full_path || '', node.name);
			check_list_modal_open = true;
		} else {
			if (!isExistFileInRootPath()) return toast.push('根目录下没有类型文件');

			await fetch(`${$sourcePath}/${$config.types_path}` || '', 'index');
			check_list_modal_open = true;
		}
	}

	async function update_request_recursive(node?: TypesTree) {
		if (node) {
			await fetch(node.full_path || '', node.name);
			if (node.children) {
				for (let subNode of node.children) {
					await recur(subNode);
				}
			}

			check_list_modal_open = true;
		} else {
			if (isExistFileInRootPath()) {
				await fetch(`${$sourcePath}/${$config.types_path}` || '', 'index');
			}

			for (let subNode of full_list) {
				await update_request_recursive(subNode);
			}
			check_list_modal_open = true;
		}

		async function recur(node: TypesTree) {
			if (!node.children.length) {
				return;
			}

			await fetch(node.full_path || '', node.name);
			if (node.children) {
				for (let subNode of node.children) {
					await recur(subNode);
				}
			}
		}
	}

	function isExistFileInRootPath() {
		return !!full_list.find((type) => !type.children.length);
	}

	async function fetch(path: string, name: string) {
		return invoke<SuccessResponse<string>>('get_request_string', { sourcePath: $sourcePath, path })
			.then((res) => {
				over_list.push({
					name,
					content: res.data,
					full_path: path,
					checked: true
				});
				over_list = over_list;
			})
			.catch((e) => {
				toast.push(JSON.stringify(e), toastTheme.error);
			});
	}
</script>

<CheckListModal bind:open={check_list_modal_open} bind:checkList={over_list} />
<main style="height:95%;display:flex ; flex-direction:column">
	<div>
		<div class="flex justify-between items-center">
			<div class="header">根据文件夹架构生成request：</div>
			<div>
				<Button on:click={() => update_request()}>更新根节点请求</Button>
				<Button on:click={() => update_request_recursive()}>递归更新所有请求</Button>
			</div>
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

	<div
		style="flex:1; overflow-y:auto;overflow-x:hidden;display:flex;flex-direction:column;gap:12px"
	>
		{#each filtered_list as item}
			<Node data={item} expanded={false} {update_request} {update_request_recursive} />
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
