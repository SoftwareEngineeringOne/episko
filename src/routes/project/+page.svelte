<script lang="ts">
	import { page } from '$app/state';
	import Commands from '$lib/commands';
	import type { Metadata } from '$lib/types';
	import { error } from '@sveltejs/kit';
	import DetailView from './DetailView.svelte';
	import ListView from './ListView.svelte';

	let id = $derived(page.url.searchParams.get('id'));

	async function loadProject(): Promise<Metadata> {
		if (!id) {
			error(400);
		}
		return await Commands.get_with_id(id);
	}
</script>

{#if !id}
	<ListView />
{:else}
	{#await loadProject()}
		<div>Loading...</div>
	{:then project}
		<DetailView {project} />
	{/await}
{/if}
