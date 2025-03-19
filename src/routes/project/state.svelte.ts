import type { MetadataPreview } from '$lib/types';

export const pageState = $state<{
	currentPage: number;
	totalPages: number;
	pageSize: number;
	loadedPreviews: MetadataPreview[];
	query: string;
}>({
	currentPage: 1,
	totalPages: 1,
	pageSize: 1,
	loadedPreviews: [],
	query: ''
});

export function resetState() {
	pageState.loadedPreviews = [];
	pageState.currentPage = 1;
}
