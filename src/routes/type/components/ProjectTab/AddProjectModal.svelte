<script lang="ts">
	import Dialog, { Header, Content, Title } from '@smui/dialog';
	import Textfield from '@smui/textfield';
	import { toast } from '@zerodevx/svelte-toast';
	import { toastTheme } from '@/consts';
	import type { Config, SuccessResponse } from '@/types/public';
	import { sourcePath } from '@/store';
	import Button, { Label } from '@smui/button';
	import { invoke } from '@tauri-apps/api';
	import type { ProjectBaseInfo } from '@/types/yapi';

	let initForm = {
		token: ''
	};

	export let open: boolean;
	export let project_list: Config['project_list'];
	export let loadProject: ()=> void

	let form = initForm;

	async function on_submit() {
		if (!form.token) {
			toast.push('请输入Yapi项目token', toastTheme.error);
			return;
		}

		try {
			toast.push('正在获取项目信息...', toastTheme.success);
			const baseInfo = await invoke<SuccessResponse<ProjectBaseInfo>>(
				'get_yapi_project_base_info',
				{
					token: form.token,
					sourcePath: $sourcePath
				}
			);

			if (
				project_list &&
				project_list.findIndex(
					(project) => String(project.project_id) === String(baseInfo.data._id)
				) > -1
			) {
				toast.push('项目已存在', toastTheme.error);
				return;
			}
			
			loadProject()

			open = false;
		} catch (e) {
			toast.push(JSON.stringify(e), toastTheme.error);
		}
	}
</script>

<Dialog bind:open fullscreen aria-labelledby="simple-title" aria-describedby="simple-content">
	<Header>
		<Title id="fullscreen-title">新增项目</Title>
		<button style="background:#fff;" on:click={() => (open = false)}>&#x2715;</button>
	</Header>
	<Content style="display:flex; flex-direction:column; gap:12px;padding-top:12px;">
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
