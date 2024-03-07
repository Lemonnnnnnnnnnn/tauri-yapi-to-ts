<script lang="ts">
	import type { InterfaceType } from '@/types/public';
	import { startTask } from '@/utils';
	import { sourcePath } from '@/store';
	import Tooltip, { Wrapper } from '@smui/tooltip';
	import { invoke } from '@tauri-apps/api';

	export let data: InterfaceType;
	export let token: string;

	async function update_interface(id: string) {
		await invoke('add_interface_task', {
			data: {
				token,
				source_path: $sourcePath,
				interface_id: id
			}
		});
		startTask();
	}
</script>

<div class="flex justify-between items-center" style="margin-bottom: 10px">
	<div>
		{data.name}
	</div>
	<div>
		<div
			style="margin-left: 10px; cursor:pointer"
			on:click|stopPropagation={() => update_interface(data.id)}
			on:keypress|stopPropagation={() => update_interface(data.id)}
			role="button"
			tabindex="0"
		>
			<Wrapper>
				<img class="icon" src="/full_update.svg" alt="更新接口" />
				<Tooltip>更新接口</Tooltip>
			</Wrapper>
		</div>
	</div>
</div>

<style>
	.icon {
		width: 20px;
		height: 20px;
	}
</style>
