<script lang="ts">
	import { Modal, TextInput, Tooltip } from 'carbon-components-svelte';
	import { request } from '@/utils';
	import { toast } from '@zerodevx/svelte-toast';
	import { toastTheme } from '@/consts';
	import type { Config, SuccessResponse } from '@/types/public';

	export let open: boolean;
	export let load_types: boolean;

	let form: Config = {
		request_path: 'src/services',
		request_template:
			"export const $1 = (params: $2) => request<$3>('/api$4' , { method: 'post' , data: params});",
		header_template: "import { request } from 'umi';",
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
		if (!form.file_name_template) {
			toast.push('请输入文件名模板字符串', toastTheme.error);
			return;
		}

		request('update_config', form)
			// @ts-expect-error
			.then((res: SuccessResponse<Config>) => {
				toast.push(res.message, toastTheme.success);
				open = false;
				load_types = true;
			})
			.catch((e) => {
				toast.push(JSON.stringify(e), toastTheme.error);
			});
	}
</script>

<Modal
	bind:open
	modalHeading="初始化配置"
	preventCloseOnClickOutside
	primaryButtonText="提交"
	secondaryButtonText="取消"
	on:click:button--secondary={() => {
		open = false;
	}}
	on:submit={on_submit}
>
	<TextInput
		value={form.request_path}
		on:change={(e) => (form.request_path = String(e.detail))}
		labelText="Request 目录文件夹路径"
		placeholder="你想要把定义 request 的 ts 文件放到哪个文件夹下？"
	/>
	<TextInput
		value={form.request_template}
		on:change={(e) => (form.request_template = String(e.detail))}
		labelText="请求模板字符串"
		placeholder="请输入请求模板字符串"
	/>
	<Tooltip align="start" direction="top">
		<p>$1: 请求名</p>
		<p>$2: 请求类型</p>
		<p>$3: 返回类型</p>
		<p>$4: 接口地址</p>
	</Tooltip>
	<TextInput
		value={form.header_template}
		on:change={(e) => (form.header_template = String(e.detail))}
		labelText="请求文件首部字符串"
		placeholder="请输入请求文件首部字符串"
	/>
	<TextInput
		value={form.file_name_template}
		on:change={(e) => (form.file_name_template = String(e.detail))}
		labelText="文件名模板字符串"
		placeholder="请输入文件名模板字符串"
	/>
	<Tooltip align="start" direction="top">
		<p>$1: 文件名</p>
	</Tooltip>
</Modal>

<style>
</style>
