<script lang="ts">
	import ProjectPreview from '$lib/components/project/preview.svelte';
	import { loadAllMetadata, metadataState } from '../metadata-state.svelte';

	let metadataPromise = loadAllMetadata();
</script>

<h1 class="text-2xl font-bold mb-4">All Projects</h1>
{#await metadataPromise}
	<h2>Loading...</h2>
{:then}
	{#each metadataState.data.values() as project (project.id)}
		<ProjectPreview {project} />
	{/each}
{/await}
<!--{#if metadataState.loading}
	<h2>Loading...</h2>
{:else}
	{#each metadataState.data.values() as project (project.id)}
		<ProjectPreview {project} />
	{/each}
{/if}-->
