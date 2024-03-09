<script lang="ts">
	import Dialog, { Content, Header, Title } from '@smui/dialog';
	import { PreviewModalContent } from '@/store';
	import { PreviewModalOpen } from '@/store';
	import { codeToHtml } from 'shiki';
	import Button from '@smui/button';
	import { writeText } from '@tauri-apps/api/clipboard';
	import { toast } from '@zerodevx/svelte-toast';
	import { toastTheme } from '@/consts';

	let highlightHtml = $PreviewModalContent;

	PreviewModalContent.subscribe(async (value) => {
		highlightHtml = await codeToHtml(value, {
			lang: 'typescript',
			theme: 'github-light'
		});
	});

	function copy(){
		writeText($PreviewModalContent).then(()=> { 
			toast.push('复制成功', toastTheme.success);
		})
	}
</script>

<Dialog
	bind:open={$PreviewModalOpen}
	fullscreen
	aria-labelledby="simple-title"
	aria-describedby="simple-content"
>
	<Header>
		<Title id="fullscreen-title">生成代码</Title>
		<button style="background:#fff;" on:click={() => ($PreviewModalOpen = false)}>&#x2715;</button>
	</Header>
	<Content style="height:350px;overflow:auto">
		<code>
			{@html highlightHtml}
		</code>
	</Content>
	<div style="margin:32px 16px 16px 16px;">
		<Button
			style="display:flex; justify-content:center;width:100%;"
			variant="raised"
			on:click={copy}>复制</Button
		>
	</div>
</Dialog>

