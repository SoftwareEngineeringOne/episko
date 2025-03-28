import type { MetadataPreview, Filter } from '$lib/types';

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

export function resetState() {
	pageState.loadedPreviews = [];
	pageState.currentPage = 1;
}
