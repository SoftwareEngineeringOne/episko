<script lang="ts">
	import ProjectPreview from '$lib/components/project/projectPreview.svelte';
	import { parseMetadataArray, type Metadata } from '$lib/schemas/metadata';

	import { invoke } from '@tauri-apps/api/core';

	let projectsPromise: Promise<Metadata[]> = invoke('all').then((data) => parseMetadataArray(data));
</script>

<h1 class="text-2xl font-bold mb-4">All Projects</h1>
{#await projectsPromise}
	<h2>Loading</h2>
{:then projects}
	{#each projects as project}
		<ProjectPreview {project} />
	{/each}
{:catch err}
	<p>Error: {err}</p>
{/await}
