<script lang="ts">
	import Commands from '$lib/commands';
	import ProjectPreview from '$lib/components/project/preview.svelte';
	import * as Pagination from '$lib/components/ui/pagination';
	import type { PagedMetadataPreview } from '$lib/types';
	import { onMount } from 'svelte';
	import { pageState } from './state.svelte';
	import { Input } from '$lib/components/ui/input';
	import Search from '@lucide/svelte/icons/search';
	import * as Accordion from '$lib/components/ui/accordion';

	async function fetchPage(page: number) {
		try {
			const pagedData: PagedMetadataPreview = await Commands.get_all(page, pageState.filter);
			pageState.loadedPreviews = pagedData.data;
			pageState.currentPage = pagedData.pageNumber;
			pageState.totalPages = pagedData.totalPages;
			pageState.pageSize = pagedData.pageSize;
		} catch (error) {
			console.log('Error fetching page data:', error);
		}
	}

	async function getCategories() {
		return await Commands.get_all_categories();
	}
	let categoriesPromise = getCategories();

	async function getLanguages() {
		return await Commands.get_all_languages();
	}
	let languagesPromise = getLanguages();

	$effect(() => {
		pageState.filter.query;
		const timeoutId = setTimeout(() => {
			fetchPage(1);
		}, 300);

		return () => clearTimeout(timeoutId);
	});

	$effect(() => {
		pageState.filter.language;
		pageState.filter.category;

		fetchPage(1);
	});

	onMount(() => {
		if (pageState.loadedPreviews.length != pageState.pageSize) {
			fetchPage(pageState.currentPage);
		}
	});
</script>

<label class="input rounded-md w-full">
	<span class="label">
		<Search />
	</span>
	<input placeholder="Search..." type="text" bind:value={pageState.filter.query} />
</label>
<div class="divider">Filters</div>
<Accordion.Root type="multiple">
	<Accordion.Item value="categories">
		<Accordion.Trigger>Categories</Accordion.Trigger>
		<Accordion.Content>
			{#await categoriesPromise}
				loading
			{:then categories}
				<form class="filter">
					<input class="btn btn-square" type="reset" value="x" />
					{#each categories as category}
						<input
							class="btn"
							type="radio"
							name="categories"
							value={category.name}
							bind:group={pageState.filter.category}
							aria-label={category.name}
						/>
					{/each}
				</form>
			{/await}
		</Accordion.Content>
	</Accordion.Item>
	<Accordion.Item value="languages">
		<Accordion.Trigger>Languages</Accordion.Trigger>
		<Accordion.Content>
			{#await languagesPromise}
				loading
			{:then languages}
				<form class="filter">
					<input class="btn btn-square" type="reset" value="x" />
					{#each languages as language}
						<input
							class="btn"
							type="radio"
							name="categories"
							value={language.name}
							bind:group={pageState.filter.language}
							aria-label={language.name}
						/>
					{/each}
				</form>
			{/await}
		</Accordion.Content>
	</Accordion.Item>
</Accordion.Root>

<div class="divider"></div>
{#each pageState.loadedPreviews as project (project.id)}
	<ProjectPreview {project} />
{/each}
<Pagination.Root
	count={pageState.totalPages}
	perPage={pageState.pageSize}
	bind:page={pageState.currentPage}
	onPageChange={fetchPage}
>
	{#snippet children({ pages, currentPage })}
		<Pagination.Content>
			<Pagination.Item>
				<Pagination.PrevButton />
			</Pagination.Item>
			{#each pages as page (page.key)}
				{#if page.type === 'ellipsis'}
					<Pagination.Item>
						<Pagination.Ellipsis />
					</Pagination.Item>
				{:else}
					<Pagination.Item>
						<Pagination.Link {page} isActive={currentPage === page.value}>
							{page.value}
						</Pagination.Link>
					</Pagination.Item>
				{/if}
			{/each}
			<Pagination.Item>
				<Pagination.NextButton />
			</Pagination.Item>
		</Pagination.Content>
	{/snippet}
</Pagination.Root>
