<script lang="ts">
	import { toastTheme } from '@/consts';
	import { request } from '@/utils';
	import { open } from '@tauri-apps/api/dialog';
	import { toast } from '@zerodevx/svelte-toast';
	import Button from '@smui/button';
	import type { SuccessResponse } from '@/types/public';

	export let need_init: boolean;
	export let load_project: boolean;

	function choose() {
		open({
			title: '选择代码库',
			directory: true,
			multiple: false
		}).then((res) => {
			request('read_config', { dir_path: res })
				// @ts-expect-error
				.then((res: SuccessResponse<null>) => {
					toast.push(JSON.stringify(res.message), toastTheme.success);
					need_init = false;
					load_project = true;
				})
				.catch((e) => {
					need_init = true;
					load_project = false;
					toast.push(JSON.stringify(e), toastTheme.error);
				});
		});
	}
</script>

<main class="flex justify-center items-center w-full" style="min-height:100vh">
	<Button variant="raised" on:click={choose}>选择代码库</Button>
</main>

<style>
</style>
