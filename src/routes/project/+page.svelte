<script lang="ts">
	import ProjectPreview from '$lib/components/project/projectPreview.svelte';
	import { onDestroy, onMount } from 'svelte';
	import { metadataState } from '../metadata-state.svelte';
	import { scrollState } from '../scroll-state.svelte';

	let scrollHandler: () => void;
	onMount(() => {
		window.scrollTo(0, scrollState.overviewPosition);

		scrollHandler = () => {
			scrollState.overviewPosition = window.scrollY;
		};

		window.addEventListener('scroll', scrollHandler);
	});

	onDestroy(() => {
		window.removeEventListener('scroll', scrollHandler);
	});
</script>

<h1 class="text-2xl font-bold mb-4">All Projects</h1>
{#if metadataState.loading}
	<h2>Loading...</h2>
{:else}
	{#each metadataState.data.values() as project (project.id)}
		<ProjectPreview {project} />
	{/each}
{/if}
