<script lang="ts">
	import Commands from '$lib/commands';
	import ProjectPreview from '$lib/components/project/preview.svelte';
	import * as Pagination from '$lib/components/ui/pagination';
	import type { PagedMetadataPreview } from '$lib/types';
	import { onMount } from 'svelte';
	import { pageState } from './state.svelte';
	import { Input } from '$lib/components/ui/input';
	import Search from '@lucide/svelte/icons/search';

	async function fetchPage(page: number) {
		try {
			const pagedData: PagedMetadataPreview = await Commands.get_all(page, pageState.query);
			pageState.loadedPreviews = pagedData.data;
			pageState.currentPage = pagedData.pageNumber;
			pageState.totalPages = pagedData.totalPages;
			pageState.pageSize = pagedData.pageSize;
		} catch (error) {
			console.log('Error fetching page data:', error);
		}
	}

	$effect(() => {
		const query = pageState.query;
		const timeoutId = setTimeout(() => {
			fetchPage(1);
		}, 300);

		return () => clearTimeout(timeoutId);
	});

	onMount(() => {
		if (pageState.loadedPreviews.length != pageState.pageSize) {
			fetchPage(pageState.currentPage);
		}
	});
</script>

<h1 class="text-2xl font-bold mb-4">All Projects</h1>
<Input id="search" placeholder="Search the docs..." class="pl-8" bind:value={pageState.query} />
<!-- Should be in search bar??? -->
<Search />
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
