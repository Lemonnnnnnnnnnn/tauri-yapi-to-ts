<script lang="ts">
	import { toastTheme } from '@/consts';
	import type { Config, GlobalConfig, SuccessResponse } from '@/types/public';
	import { toast } from '@zerodevx/svelte-toast';
	import Tooltip, { Wrapper } from '@smui/tooltip';
	import Textfield from '@smui/textfield';

	import { onMount } from 'svelte';
	import { sourcePath } from '@/store';
	import { invoke } from '@tauri-apps/api';

	let initProjectConfig: Config = {
		base_url: '',
		types_path: '',
		request_path: '',
		request_template: '',
		header_template: '',
		file_name_template: '',
		type_import_template: ''
	};

	let initGlobalConfig: GlobalConfig = {
		proxy: '',
		rate_limit: 5,
		break_seconds: 1
	};

	let projectConfig = initProjectConfig;
	let globalConfig = initGlobalConfig;

	onMount(() => {
		invoke<SuccessResponse<GlobalConfig>>('load_global_config', {}).then((res) => {
			console.log(res);

			globalConfig = res.data;
			console.log(globalConfig);
		});

		invoke<SuccessResponse<Config>>('load_project_config', { sourcePath: $sourcePath }).then(
			(res) => {
				projectConfig = res.data;
			}
		);
	});

	function update_project_config() {
		invoke<SuccessResponse<String>>('update_project_config', {
			sourcePath: $sourcePath,
			data: projectConfig
		}).catch((e) => {
			toast.push(JSON.stringify(e), toastTheme.error);
		});
	}

	function update_global_config() {
		invoke<SuccessResponse<String>>('update_global_config', {
			data: globalConfig
		}).catch((e) => {
			toast.push(JSON.stringify(e), toastTheme.error);
		});
	}
</script>

<main
	style="height:500px; overflow:auto; display:flex;flex-direction: column; gap:16px; padding-top:24px"
>
	<div>
		<Textfield
			style="width:100%"
			variant="outlined"
			bind:value={projectConfig.base_url}
			label="Yapi地址根路径"
			on:blur={() => {
				update_project_config();
			}}
		></Textfield>
	</div>
	<div>
		<Textfield
			style="width:100%"
			variant="outlined"
			bind:value={projectConfig.types_path}
			label="你想要把接口ts文件放到哪个文件夹下？"
			on:blur={() => {
				update_project_config();
			}}
		></Textfield>
	</div>
	<div>
		<Textfield
			style="width:100%"
			type="number"
			variant="outlined"
			bind:value={globalConfig.rate_limit}
			label="请求yapi-openapi最大并行请求数"
			on:blur={() => {
				update_global_config();
			}}
		></Textfield>
	</div>
	<div>
		<Textfield
			style="width:100%"
			type="number"
			variant="outlined"
			bind:value={globalConfig.break_seconds}
			label="请求yapi-openapi时间间隔"
			on:blur={() => {
				update_global_config();
			}}
		></Textfield>
	</div>
	<div>
		<Textfield
			style="width:100%"
			variant="outlined"
			bind:value={projectConfig.request_path}
			label="你想要把定义 request 的 ts 文件放到哪个文件夹下？"
			on:blur={() => {
				update_project_config();
			}}
		></Textfield>
	</div>

	<div>
		<Wrapper>
			<Textfield
				style="width:100%"
				variant="outlined"
				bind:value={projectConfig.request_template}
				label="请求模板字符串"
				on:blur={() => {
					update_project_config();
				}}
			></Textfield>
			<Tooltip>
				<p>$1: 请求名</p>
				<p>$2: 请求类型</p>
				<p>$3: 返回类型</p>
				<p>$4: 接口地址</p>
			</Tooltip>
		</Wrapper>
	</div>
	<div>
		<Wrapper>
			<Textfield
				style="width:100%"
				variant="outlined"
				bind:value={projectConfig.type_import_template}
				label="import类型模板"
				on:blur={() => {
					update_project_config();
				}}
			></Textfield>
			<Tooltip>
				<p>$1: Request Type 类型</p>
				<p>$2: Response Type 类型</p>
				<p>$3: 类型文件相对地址（请在前面添加类型文件夹别名）</p>
			</Tooltip>
		</Wrapper>
	</div>

	<div>
		<Textfield
			style="width:100%"
			variant="outlined"
			bind:value={projectConfig.header_template}
			label="请求文件首部字符串"
			on:blur={() => {
				update_project_config();
			}}
		></Textfield>
	</div>

	<div>
		<Wrapper>
			<Textfield
				style="width:100%"
				variant="outlined"
				bind:value={projectConfig.file_name_template}
				label="文件名模板字符串"
				on:blur={() => {
					update_project_config();
				}}
			></Textfield>

			<Tooltip>
				<p>$1: 文件名</p>
			</Tooltip>
		</Wrapper>
	</div>

	<div>
		<Textfield
			style="width:100%"
			variant="outlined"
			bind:value={globalConfig.proxy}
			label="代理地址"
			on:blur={() => {
				update_global_config();
			}}
		></Textfield>
	</div>
</main>

<style>
</style>
