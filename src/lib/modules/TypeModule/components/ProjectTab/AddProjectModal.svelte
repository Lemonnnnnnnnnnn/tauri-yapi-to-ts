<script lang="ts">
	import Dialog, { Actions, Header, Content, Title } from '@smui/dialog';
	import Textfield from '@smui/textfield';
	import { request } from '@/utils';
	import { toast } from '@zerodevx/svelte-toast';
	import { toastTheme } from '@/consts';
	import type { SuccessResponse } from '@/types/public';
	import { runningTask, processingModalOpen, processingModalTotal } from '@/store';
	import { confirm } from '@tauri-apps/api/dialog';
	import Button, { Label } from '@smui/button';

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

<Dialog bind:open fullscreen aria-labelledby="simple-title" aria-describedby="simple-content">
	<Header>
		<Title id="fullscreen-title">新增项目</Title>
		<button style="background:#fff;" on:click={() => (open = false)}>&#x2715;</button>
	</Header>
	<Content
		id="fullscreen-content"
		style="display:flex; flex-direction:column; gap:12px;padding-top:12px;"
	>
		<Textfield variant="outlined" bind:value={form.id} label="Yapi项目ID"></Textfield>
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
