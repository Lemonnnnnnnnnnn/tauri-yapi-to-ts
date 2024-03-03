<script lang="ts">
	import { toastTheme } from '@/consts';
	import { open } from '@tauri-apps/api/dialog';
	import { toast } from '@zerodevx/svelte-toast';
	import Button from '@smui/button';
	import type { SuccessResponse } from '@/types/public';
	import { invoke } from '@tauri-apps/api';

	function choose() {
		open({
			title: '选择代码库',
			directory: true,
			multiple: false
		}).then((res) => {
			invoke<SuccessResponse<null>>('add_project', { sourcePath: res })
				.then((res) => {
					toast.push(JSON.stringify(res.message), toastTheme.success);
				})
				.catch((e) => {
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
