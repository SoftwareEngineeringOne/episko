<script lang="ts">
	import { goto } from '$app/navigation';
	import ProjectPreview from '$lib/components/project/projectPreview.svelte';
	import type { PageProps } from './$types';

	let { data }: PageProps = $props();
	let promise = data.projectPromise;
</script>

{#await promise}
	<h2>Loading...</h2>
{:then project}
	<a href="/project">Back</a>
	<h1>{project.title}</h1>
	<p>All details can be displayed here</p>
{:catch err}
	<p>ERROR: ${err}</p>
{/await}
