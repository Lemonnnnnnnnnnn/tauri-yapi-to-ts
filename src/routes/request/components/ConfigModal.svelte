<script lang="ts">
	import Textfield from '@smui/textfield';
	import Tooltip, { Title, Wrapper } from '@smui/tooltip';
	import { request } from '@/utils';
	import { toast } from '@zerodevx/svelte-toast';
	import { toastTheme } from '@/consts';
	import type { Config, SuccessResponse } from '@/types/public';
	import Dialog, { Actions, Header, Content } from '@smui/dialog';
	import Button, { Label } from '@smui/button';
	import { sourcePath } from '@/store';
	import { invoke } from '@tauri-apps/api';

	export let open: boolean;
	export let load_types: boolean;

	let form: Config = {
		request_path: 'src/services',
		request_template:
			"export const $1 = (params: $2) => request<$3>('/api$4' , { method: 'post' , data: params});",
		header_template: "import { request } from 'umi';",
		type_import_template: 'import { type $1 , type $2 } from "@/types/$3"',
		file_name_template: '$1'
	};

	function on_submit() {
		if (!form.request_path) {
			toast.push('请输入Request 目录文件夹路径', toastTheme.error);
			return;
		}
		if (!form.request_template) {
			toast.push('请输入请求模板字符串', toastTheme.error);
			return;
		}
		if (!form.header_template) {
			toast.push('请输入请求文件首部字符串', toastTheme.error);
			return;
		}
		if (!form.type_import_template) {
			toast.push('请输入import类型模板', toastTheme.error);
			return;
		}
		if (!form.file_name_template) {
			toast.push('请输入文件名模板字符串', toastTheme.error);
			return;
		}

		invoke<SuccessResponse<String>>('update_project_config', {
			sourcePath: $sourcePath,
			data: form
		})
			.then((res) => {
				toast.push(res.message, toastTheme.success);
				open = false;
				load_types = true;
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
		<Textfield
			variant="outlined"
			bind:value={form.request_path}
			label="你想要把定义 request 的 ts 文件放到哪个文件夹下？"
		></Textfield>
		<Wrapper>
			<Textfield variant="outlined" bind:value={form.request_template} label="请求模板字符串"
			></Textfield>
			<Tooltip>
				<p>$1: 请求名</p>
				<p>$2: 请求类型</p>
				<p>$3: 返回类型</p>
				<p>$4: 接口地址</p>
			</Tooltip>
		</Wrapper>

		<Wrapper>
			<Textfield variant="outlined" bind:value={form.type_import_template} label="import类型模板"
			></Textfield>
			<Tooltip>
				<p>$1: Request Type 类型</p>
				<p>$2: Response Type 类型</p>
				<p>$3: 类型文件相对地址（请在前面添加类型文件夹别名）</p>
			</Tooltip>
		</Wrapper>
		<Textfield variant="outlined" bind:value={form.header_template} label="请求文件首部字符串"
		></Textfield>
		<Wrapper>
			<Textfield variant="outlined" bind:value={form.file_name_template} label="文件名模板字符串"
			></Textfield>

			<Tooltip>
				<p>$1: 文件名</p>
			</Tooltip>
		</Wrapper>
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
