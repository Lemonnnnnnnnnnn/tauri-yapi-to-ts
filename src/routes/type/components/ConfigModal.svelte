<script lang="ts">
	import Dialog, { Header, Content, Title } from '@smui/dialog';
	import Textfield from '@smui/textfield';
	import { toast } from '@zerodevx/svelte-toast';
	import { toastTheme } from '@/consts';
	import type { Config, SuccessResponse } from '@/types/public';
	import Button, { Label } from '@smui/button';
	import { invoke } from '@tauri-apps/api';
	import { sourcePath } from '@/store';

	export let open: boolean;
	export let load_project: boolean;
	export let loadProject: ()=> void

	let init_form: Config = {
		base_url: '',
		types_path: 'src/types'
	};

	$: form = init_form;

	function on_submit() {
		if (!form.base_url) {
			toast.push('请输入Yapi地址根路径', toastTheme.error);
			return;
		}
		const httpReg = /^https?:\/\//;
		if (!httpReg.test(form.base_url)) {
			toast.push('Yapi地址必须是 http/https 的网络地址', toastTheme.error);
			return;
		}

		if (!form.types_path) {
			toast.push('请输入项目类型目录文件夹路径', toastTheme.error);
			return;
		}

		invoke<SuccessResponse<Config>>('update_project_config', {
			sourcePath: $sourcePath,
			data: form
		})
			.then((res) => {
				toast.push(res.message, toastTheme.success);
				// open = false;
				// load_project = true;
				loadProject();
			})
			.catch((e) => {
				toast.push(JSON.stringify(e), toastTheme.error);
			});
	}
</script>

<Dialog bind:open fullscreen aria-labelledby="simple-title" aria-describedby="simple-content">
	<Header>
		<Title id="fullscreen-title">初始化配置</Title>
		<button style="background:#fff;" on:click={() => (open = false)}>&#x2715;</button>
	</Header>
	<Content
		id="fullscreen-content"
		style="display:flex; flex-direction:column; gap:12px;padding-top:12px;"
	>
		<Textfield variant="outlined" bind:value={form.base_url} label="Yapi地址根路径"></Textfield>
		<Textfield
			variant="outlined"
			bind:value={form.types_path}
			label="你想要把接口ts文件放到哪个文件夹下？"
		></Textfield>
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

<style>
</style>
