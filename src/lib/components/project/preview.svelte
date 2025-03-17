<script lang="ts">
	import * as Card from '$lib/components/ui/card/index';
	import { Badge } from '$lib/components/ui/badge/index.js';
	import type { Metadata } from '$lib/types';
	import { goto } from '$app/navigation';

	let { project }: { project: Metadata } = $props();

	let seeDetails = () => {
		goto(`/project/${project.id}`);
	};
</script>

<Card.Root class="transition hover:-translate-y-1 hover:cursor-pointer" onclick={seeDetails}>
	<Card.Header>
		<Card.Title>{project.title}</Card.Title>
		<Card.Description>Created on {project.created.toLocaleDateString()}</Card.Description>
	</Card.Header>
	{#if project.description}
		<Card.Content>
			<p>{project.description}</p>
		</Card.Content>
	{:else}
		<br />
	{/if}
	<Card.Footer>
		{#each project.languages as language}
			<Badge>{language.name}</Badge>
			<p>&nbsp;</p>
		{/each}
	</Card.Footer>
</Card.Root>
