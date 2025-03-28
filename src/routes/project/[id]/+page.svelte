<script lang="ts">
	import { goto } from '$app/navigation';
	import Badge from '$lib/components/ui/badge/badge.svelte';
	import { Button } from '$lib/components/ui/button';
	import { Separator } from '$lib/components/ui/separator';
	import type { Uuid } from '$lib/types';
	import { ArrowLeft, GitGraph, Pencil } from 'lucide-svelte';
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

<div class="w-full">
	<Button onclick={goBack} variant="link">
		<ArrowLeft />
		Back
	</Button>
	<div class="m-4 flex flex-row w-full gap-5">
		<div class="flex flex-col">
			<div>
				<h1 class="text-5xl font-bold">{project.title}</h1>
				<div class="flex flex-rol gap-4">
					<p class="text-sm opacity-60">Last Updated {project.updated.toLocaleTimeString()}</p>
					{#if true || project.repositoryUrl}
						<a href={project.repositoryUrl} class="hover:cursor-pointer hover:underline">
							<div class="flex flex-row gap-1 text-primary text-sm items-center">
								<GitGraph size={18} />
								<p>Repository</p>
							</div>
						</a>
					{/if}
				</div>
			</div>
			<Separator class="m-3" />
			<div>
				<p class="text-lg">{project.description}</p>
			</div>
			<div>
				{#if project.categories.length > 0}
					<Separator class="m-3" />
					{#each project.categories as category}
						<Badge class="ml-2">{category.name}</Badge>
					{/each}
				{/if}
			</div>
		</div>
		<div class="divider divider-horizontal"></div>
		<div class="flex flex-col gap-5">
			{#if project.languages.length > 0}
				<div>
					<h2 class="text-sm opacity-60 font-semibold">Languages</h2>
					<div class="flex flex-col gap-1">
						{#each project.languages as language}
							<Badge>{language.name} {language.version}</Badge>
						{/each}
					</div>
				</div>
			{/if}

			{#if project.buildSystems.length > 0}
				<div>
					<h2 class="text-sm opacity-60 font-semibold">Build Systems</h2>
					<div class="flex flex-col gap-1">
						{#each project.buildSystems as buildSystem}
							<Badge>{buildSystem.name} {buildSystem.version}</Badge>
						{/each}
					</div>
				</div>
			{/if}

			{#if project.preferredIde}
				<div>
					<h2 class="text-sm opacity-60 font-semibold">Preferred IDE</h2>
					<p>{project.preferredIde.name}</p>
				</div>
			{/if}
		</div>
	</div>
	<Button class="absolute right-4 bottom-4" onclick={edit(project.id)} variant="secondary">
		<Pencil />
		<p class="text-xl opacity-60">Edit</p>
	</Button>
</div>
