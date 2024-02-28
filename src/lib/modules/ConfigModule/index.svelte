<script lang="ts">
	import { toastTheme } from '@/consts';
	import type { Config, SuccessResponse } from '@/types/public';
	import { request } from '@/utils';
	import { toast } from '@zerodevx/svelte-toast';
	import Tooltip, { Wrapper } from '@smui/tooltip';
	import Textfield from '@smui/textfield';
	import Button from '@smui/button';
	

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
		proxy: ''
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

<main
	style="height:500px; overflow:auto; display:flex;flex-direction: column;gap:18px; padding-top:24px"
>
	<Textfield variant="outlined" bind:value={form.source_path} label="本地项目根路径"></Textfield>
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

	<Textfield variant="outlined" bind:value={form.proxy} label="代理地址"
	></Textfield>

</main>
<footer class="config-module-footer">
	<Button variant="raised" on:click={update_config}>提交</Button>
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
