<script lang="ts">
	import { goto } from '$app/navigation';
	import Badge from '$lib/components/ui/badge/badge.svelte';
	import { Button } from '$lib/components/ui/button';
	import { Separator } from '$lib/components/ui/separator';
	import type { Uuid } from '$lib/types';
	import type { PageProps } from './$types';

	let { data }: PageProps = $props();
	let project = data.project;

	console.log('Project: ', project);

	function goBack() {
		history.back();
	}

	function edit(id: Uuid) {
		return () => {
			goto(`/project/${id}/edit`);
		};
	}
</script>

<div>
	<Button onclick={goBack} variant="link">Back</Button>
	<h1 class="text-xl">{project.title}</h1>
	<p>{project.description}</p>
	{#if project.categories.length > 0}
		<Separator class="m-3" />
		<h2>Categories</h2>
		{#each project.categories as category}
			<Badge>{category.name}</Badge>
		{/each}
	{/if}

	{#if project.languages.length > 0}
		<Separator class="m-3" />
		<h2>Languages</h2>
		{#each project.languages as language}
			<Badge>{language.name} {language.version}</Badge>
		{/each}
	{/if}

	{#if project.buildSystems.length > 0}
		<Separator class="m-3" />
		<h2>Build Systems</h2>
		{#each project.buildSystems as buildSystem}
			<Badge>{buildSystem.name} {buildSystem.version}</Badge>
		{/each}
	{/if}

	{#if project.preferredIde}
		<Separator class="m-3" />
		<h2>Preferred Ide</h2>
		<p>{project.preferredIde.name}</p>
	{/if}

	{#if project.repositoryUrl}
		<Separator class="m-3" />
		<a href={project.repositoryUrl}>Repository</a>
	{/if}
	<Separator class="m-3" />
	<Button onclick={edit(project.id)}>Edit</Button>
</div>
