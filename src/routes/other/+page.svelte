<script lang="ts">
	import { onMount } from 'svelte';
	import { appLogDir, appLocalDataDir } from '@tauri-apps/api/path';
	import { invoke } from '@tauri-apps/api';
	import type { GlobalConfig, SuccessResponse } from '@/types/public';
	import Tooltip, { Wrapper } from '@smui/tooltip';
	import { toast } from '@zerodevx/svelte-toast';
	import { toastTheme } from '@/consts';
	import { open } from '@tauri-apps/api/dialog';
	import { loadConfig } from '@/utils';
	import { sourcePath } from '@/store';
	import Button from '@smui/button';

	
	let logDir = '';
	let configDir = '';
	let projects: string[] = [];

	onMount(() => {
		appLogDir().then((res) => {
			logDir = res;
		});

		appLocalDataDir().then((res) => {
			configDir = res;
		});

		invoke<SuccessResponse<GlobalConfig>>('load_global_config', {}).then((res) => {
			projects = res.data.projects || [];
		});
	});

	function loadProject(path: string) {
		invoke<SuccessResponse<null>>('update_project', { sourcePath: path })
			.then((_) => {
				toast.push('切换项目成功', toastTheme.success);
			})
			.catch((e) => {
				toast.push(JSON.stringify(e), toastTheme.error);
			});
	}
	function mergeYapiConfig() {
		open({
			title: '选择要合并的配置文件',
			multiple: false
		}).then((res) => {
			if (res) {
				invoke<SuccessResponse<null>>('merge_project_config', {
					sourcePath: $sourcePath,
					otherConfigPath: res
				})
					.then((res) => {
						toast.push(JSON.stringify(res.message), toastTheme.success);
						loadConfig($sourcePath);
					})
					.catch((e) => {
						toast.push(JSON.stringify(e), toastTheme.error);
					});
			}
		});
	}

	function exportYapiConfig() {
		open({
			title: '选择要导出到的位置',
			directory:true,
			multiple: false
		}).then((res) => {
			if (res) {
				invoke<SuccessResponse<null>>('export_project_config', {
					sourcePath: $sourcePath,
					targetPath: res
				})
					.then((res) => {
						toast.push(JSON.stringify(res.message), toastTheme.success);
					})
					.catch((e) => {
						toast.push(JSON.stringify(e), toastTheme.error);
					});
			}
		});
	}
</script>

<div style="padding:16px">
	<h2>本地项目列表</h2>
	<div class="card-wrapper">
		{#each projects as project}
			<button on:click={() => loadProject(project)} class="card">
				<Wrapper>
					<div class="card-text">{project}</div>
					<Tooltip>{project}</Tooltip>
				</Wrapper>
			</button>
		{/each}
	</div>

	<h2>工程配置文件</h2>
	<Button on:click={mergeYapiConfig}>合并配置文件</Button>
	<Button on:click={exportYapiConfig}>导出配置文件</Button>


	<h2>全局配置文件地址</h2>
	<div>日志文件夹：{logDir}</div>
	<br />
	<div>配置文件夹：{configDir}</div>
</div>

<style>
	.card-wrapper {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(256px, 1fr));
		gap: 16px;
	}
	.card {
		border: 1px solid #ccc;
		padding: 16px;
		margin-bottom: 16px;
		box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
		cursor: pointer;
		background-color: #fff;
	}

	.card-text {
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}
</style>
