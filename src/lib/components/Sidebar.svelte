<script lang="ts">
	import { page } from '$app/stores';

	let menuList = [
		{
			key: '1',
			label: 'Type',
			url: '/type'
		},
		{
			key: '2',
			label: 'Request',
			url: '/request'
		},
		{
			key: '3',
			label: 'Config',
			url: '/config'
		}
	];

	$: currentPage = $page.url.pathname;
</script>

<aside>
	{#each menuList as item}
		<a href={item.url} class="menu-item" class:active={currentPage.includes(item.url)}>
			{item.label}
		</a>
	{/each}
</aside>

<style>
	aside {
		width: 150px;
		height: 100vh;
		border-right: 1px solid rgba(0, 0, 0, 0.1);
		display: flex;
		flex-direction: column;
	}

	.menu-item {
		padding: 10px;
		transition: all 0.2s ease-in-out;
		cursor: pointer;
		text-align: center;
		text-decoration: none;
		color: #333;
	}

	.menu-item.active {
		font-weight: bold;
		transform: scale(1.1);
	}

	.menu-item:not(:last-child) {
		border-bottom: 1px solid rgba(0, 0, 0, 0.1);
	}
</style>
