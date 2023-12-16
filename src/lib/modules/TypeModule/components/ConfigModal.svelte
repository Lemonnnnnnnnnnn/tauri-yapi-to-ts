<script lang="ts">
	import { Modal, TextInput } from 'carbon-components-svelte';
	import { request } from '@/utils';
	import { toast } from '@zerodevx/svelte-toast';
	import { toastTheme } from '@/consts';
	import type { Config, SuccessResponse } from '@/types/public';

	export let open: boolean;
	export let load_project: boolean;

	let init_form: Config = {
		base_url: '',
		types_path: 'src/types',
		rate_limit: 5,
		break_seconds: 0
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

<Modal
	bind:open
	modalHeading="初始化配置"
	preventCloseOnClickOutside
	selectorPrimaryFocus="#source-code"
	primaryButtonText="提交"
	secondaryButtonText="取消"
	on:click:button--secondary={() => {
		open = false;
	}}
	on:submit={on_submit}
>
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
</Modal>

<style>
</style>
