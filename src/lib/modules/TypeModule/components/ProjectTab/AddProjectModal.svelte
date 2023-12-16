<script lang="ts">
	import { Modal, TextInput, ComposedModal } from 'carbon-components-svelte';
	import { request } from '@/utils';
	import { toast } from '@zerodevx/svelte-toast';
	import { toastTheme } from '@/consts';
	import type { QueueStatus, SuccessResponse } from '@/types/public';
	import { runningTask, processingModalOpen, processingModalTotal } from '@/store';
	import { confirm } from '@tauri-apps/api/dialog';

	let initForm = {
		id: '',
		token: ''
	};

	export let open: boolean;
	export let banInitModal: boolean;

	let form = initForm;

	async function on_submit() {
		if ($runningTask) {
			toast.push('正在执行任务...请稍等', toastTheme.error);
			return;
		}

		if (!form.id) {
			toast.push('请输入Yapi项目ID', toastTheme.error);
			return;
		}

		if (!form.token) {
			toast.push('请输入Yapi项目token', toastTheme.error);
			return;
		}

		const confirmed = await confirm('操作将生成ts文件，是否确定？');

		if (!confirmed) return;

		toast.push('正在添加任务...');
		request('update_projects', { projects: [form], is_full_update: true })
			// @ts-expect-error
			.then((res: SuccessResponse<number>) => {
				if (res.data === 0) {
					toast.push('无待执行的任务');
				} else {
					toast.push(res.message, toastTheme.success);
					processingModalOpen.update(() => true);
					processingModalTotal.update(() => res.data);
					runningTask.update(() => true);
				}
				banInitModal = true;
				open = false;
				form = initForm;
			})
			.catch((e) => {
				toast.push(JSON.stringify(e), toastTheme.error);
			});
	}
</script>

<Modal
	bind:open
	preventCloseOnClickOutside
	modalHeading="新增项目"
	selectorPrimaryFocus="#source-code"
	primaryButtonText="提交"
	secondaryButtonText="取消"
	on:click:button--secondary={() => {
		open = false;
	}}
	on:submit={on_submit}
>
	<TextInput
		value={form.id}
		on:change={(e) => (form.id = String(e.detail))}
		labelText="Yapi项目ID"
		placeholder="请输入Yapi项目ID"
	/>
	<TextInput
		value={form.token}
		on:change={(e) => (form.token = String(e.detail))}
		labelText="Yapi项目token"
		placeholder="请输入Yapi项目token"
	/>
</Modal>
