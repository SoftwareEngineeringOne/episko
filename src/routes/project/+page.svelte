<script lang="ts">
	import ProjectPreview from '$lib/components/project/preview.svelte';
	import { metadataState } from '../metadata-state.svelte';
</script>

<h1 class="text-2xl font-bold mb-4">All Projects</h1>
{#if metadataState.loading}
	<h2>Loading...</h2>
{:else}
	{#each metadataState.data.values() as project (project.id)}
		<ProjectPreview {project} />
	{/each}
{/if}
