<script lang="ts">
	import { onMount } from 'svelte';
	import { appLogDir, appLocalDataDir } from '@tauri-apps/api/path';

	let logDir = '';
	let configDir = '';

	onMount(() => {
		appLogDir().then((res) => {
			logDir = res;
		});

		appLocalDataDir().then((res) => {
			configDir = res;
		});
	});
</script>

<div style="padding:16px">
	<div>日志文件夹：{logDir}</div>
	<br />
	<div>配置文件夹：{configDir}</div>
</div>
