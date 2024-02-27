<script lang="ts">
	import { toastTheme } from '@/consts';
	import type { Config, SuccessResponse } from '@/types/public';
	import { request } from '@/utils';
	import { toast } from '@zerodevx/svelte-toast';
	import { Button, TextInput, Tooltip } from 'carbon-components-svelte';
	import { onMount } from 'svelte';

	let init_form: Config = {
		source_path: '',
		base_url: '',
		rate_limit: 0,
		types_path: '',
		types_full_path: '',
		break_seconds: 0,
		request_path: '',
		request_template: '',
		header_template: '',
		file_name_template: '',
		type_import_template: '',
		proxy:''
	};

	let form = init_form;

	onMount(() => {
		// @ts-expect-error
		request('get_config', {}).then((res: SuccessResponse<Config>) => {
			form = res.data;
		});
	});

	function update_config() {
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

		if (!form.type_import_template) {
			toast.push('请输入import类型模板', toastTheme.error);
			return;
		}

		// @ts-expect-error
		request('update_config', form).then((res: SuccessResponse<Config>) => {
			toast.push(res.message, toastTheme.success);
		});
	}
</script>

<main style="height:500px; overflow:auto">
	<TextInput
		value={form.source_path}
		on:change={(e) => (form.source_path = String(e.detail))}
		labelText="本地项目根路径"
		placeholder="请输入本地项目根路径"
	/>
	<TextInput
		value={form.base_url}
		on:change={(e) => (form.base_url = String(e.detail))}
		labelText="Yapi地址根路径"
		placeholder="请输入Yapi地址根路径"
	/>
	<TextInput
		value={form.types_path}
		on:change={(e) => (form.types_path = String(e.detail))}
		labelText="项目类型目录文件夹路径"
		placeholder="你想要把接口ts文件放到哪个文件夹下？"
	/>
	<TextInput
		value={form.rate_limit}
		on:change={(e) => (form.rate_limit = Number(e.detail))}
		labelText="请求yapi-openapi最大并行请求数"
		placeholder="请输入请求yapi-openapi最大并行请求数"
		type="number"
	/>
	<TextInput
		value={form.break_seconds}
		on:change={(e) => (form.break_seconds = Number(e.detail))}
		labelText="请求yapi-openapi时间间隔"
		placeholder="请输入请求yapi-openapi时间间隔"
		type="number"
	/>

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
		value={form.type_import_template}
		on:change={(e) => (form.type_import_template = String(e.detail))}
		labelText="import类型模板"
		placeholder="请输入import类型模板"
	/>
	<Tooltip align="start" direction="top">
		<p>$1: Request Type 类型</p>
		<p>$2: Response Type 类型</p>
		<p>$3: 类型文件相对地址（请在前面添加类型文件夹别名）</p>
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
	<TextInput
		value={form.proxy}
		on:change={(e) => (form.proxy = String(e.detail))}
		labelText="代理地址"
		placeholder="请输入代理地址"
	/>
</main>
<footer class="config-module-footer">
	<Button type="primary" on:click={update_config}>提交</Button>
</footer>

<style>
	.config-module-footer :global(.bx--btn) {
		width: 100%;
		max-width: 100vw;
		text-align: center;
		display: block;
		margin-top: 20px;
	}
</style>
