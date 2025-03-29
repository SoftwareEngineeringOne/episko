import Commands from '$lib/commands';
import type { MetadataPreview, Filter, PagedMetadataPreview } from '$lib/types';

export const pageState = $state<{
	currentPage: number;
	totalPages: number;
	pageSize: number;
	loadedPreviews: MetadataPreview[];
	filter: Filter;
}>({
	currentPage: 1,
	totalPages: 1,
	pageSize: 1,
	loadedPreviews: [],
	filter: {
		query: '',
		category: null,
		language: null
	}
});

export async function preloadFirstPage() {
	const pagedData: PagedMetadataPreview = await Commands.get_all(1, pageState.filter);
	pageState.loadedPreviews = pagedData.data;
	pageState.currentPage = pagedData.pageNumber;
	pageState.totalPages = pagedData.totalPages;
	pageState.pageSize = pagedData.pageSize;
}

export function resetState() {
	pageState.loadedPreviews = [];
	pageState.currentPage = 1;
}
