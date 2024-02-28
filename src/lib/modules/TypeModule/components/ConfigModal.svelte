<script lang="ts">
	import Dialog, { Actions, Header, Content, Title } from '@smui/dialog';
	import Textfield from '@smui/textfield';
	import { request } from '@/utils';
	import { toast } from '@zerodevx/svelte-toast';
	import { toastTheme } from '@/consts';
	import type { Config, SuccessResponse } from '@/types/public';
	import Button, { Label } from '@smui/button';

	export let open: boolean;
	export let load_project: boolean;

	let init_form: Config = {
		base_url: '',
		types_path: 'src/types',
		rate_limit: 10,
		break_seconds: 1
	};

	$: form = init_form;

	function on_submit() {
		if (!form.base_url) {
			toast.push('请输入Yapi地址根路径', toastTheme.error);
			return;
		}
		if (!form.types_path) {
			toast.push('请输入项目类型目录文件夹路径', toastTheme.error);
			return;
		}
		if (!form.rate_limit) {
			toast.push('请输入请求yapi-openapi最大并行请求数', toastTheme.error);
			return;
		}
		if (form.break_seconds === undefined) {
			toast.push('请输入请求yapi-openapi时间间隔', toastTheme.error);
			return;
		}
		request('update_config', form)
			// @ts-expect-error
			.then((res: SuccessResponse<Config>) => {
				toast.push(res.message, toastTheme.success);
				open = false;
				load_project = true;
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
		<Textfield
			type="number"
			variant="outlined"
			bind:value={form.rate_limit}
			label="请求yapi-openapi最大并行请求数"
		></Textfield>
		<Textfield
			type="number"
			variant="outlined"
			bind:value={form.break_seconds}
			label="请求yapi-openapi时间间隔"
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
