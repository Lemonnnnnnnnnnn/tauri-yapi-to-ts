<script lang="ts">
	import { toastTheme } from '@/consts';
	import { PreviewModalContent, PreviewModalOpen, sourcePath } from '@/store';
	import type { SuccessResponse, TypesTree } from '@/types/public';
	import { wop } from '@/utils';
	import Tooltip, { Wrapper } from '@smui/tooltip';
	import { invoke } from '@tauri-apps/api';
	import { toast } from '@zerodevx/svelte-toast';

	export let expanded = false;
	export let level = 1;
	export let data: TypesTree;
	export let update_request_recursive: (node?: TypesTree) => any;
	export let update_request: (node?: TypesTree) => any;

	function toggle() {
		if (data.children.length == 0) {
			return;
		}
		expanded = !expanded;
	}

	function preview(data: TypesTree) {
		invoke<SuccessResponse<string>>('get_request_string', {
			sourcePath: $sourcePath,
			path: data.full_path
		})
			.then((res) => {
				$PreviewModalOpen = true;
				$PreviewModalContent = res.data;
				toast.push(JSON.stringify(res.message), toastTheme.success);
			})
			.catch((e) => {
				toast.push(JSON.stringify(e), toastTheme.error);
			});
	}
</script>

<div class:wrapper={level == 2}>
	<div class="flex-inline items-end">
		<button class="flex-inline items-end" on:click={toggle}>
			{#if data.children.length}
				<img class="icon" src="/directory.svg" alt="directory" />
			{:else}
				<img class="icon" src="/file.svg" alt="file" />
			{/if}
			<button class="node" class:expanded>{data.name}</button>
		</button>
		{#if data.children.length}
			<button
				class="flex-inline items-end"
				style="margin-left:1em"
				on:click={() => update_request(data)}
			>
				<Wrapper>
					<img class="icon" src="/update.svg" alt="生成该文件夹下的 type 文件对应的 request" />
					<Tooltip>生成该文件夹下的 type 文件对应的 request</Tooltip>
				</Wrapper>
			</button>
			<button
				class="flex-inline items-end"
				style="margin-left:1em"
				on:click={() => update_request_recursive(data)}
			>
				<Wrapper>
					<img
						class="icon"
						src="/full_update.svg"
						alt="递归生成该文件夹下的 type 文件对应的 request"
					/>
					<Tooltip>递归生成该文件夹下的 type 文件对应的 request</Tooltip>
				</Wrapper>
			</button>
			<button class="flex-inline items-end" style="margin-left:1em" on:click={() => preview(data)}>
				<Wrapper>
					<img class="icon" src="/preview.svg" alt="查看生成的代码" />
					<Tooltip>查看生成的代码</Tooltip>
				</Wrapper>
			</button>
		{/if}
	</div>

	{#if expanded}
		<ul transition:wop>
			{#each data.children as child}
				<li>
					<svelte:self data={child} {update_request_recursive} {update_request} level={level + 1} />
				</li>
			{/each}
		</ul>
	{/if}
</div>

<style>
	.wrapper {
		border: 1px solid #78bdcc;
		border-radius: 6px;
		padding: 8px;
		display: inline-block;
	}

	.node {
		padding: 0 0 0 1em;
		background-size: 1em 1em;
		font-size: 20px;
	}

	button {
		background: #fff;
		cursor: pointer;
		border: none;
		padding: 0;
	}

	.expanded {
		font-weight: bold;
	}

	ul {
		padding: 0.2em 0 0 0.5em;
		margin: 0 0 0 0.5em;
		list-style: none;
		border-left: 1px solid #eee;
		transition: all 1s ease-in;
	}

	li {
		padding: 0.2em 0;
	}

	.icon {
		width: 20px;
		height: 20px;
	}
</style>
