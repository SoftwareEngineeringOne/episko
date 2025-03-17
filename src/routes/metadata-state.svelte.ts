import Commands from '$lib/commands';
import { SvelteMap } from 'svelte/reactivity';
import type { Metadata, Uuid } from '$lib/types';

export const metadataState = $state<{
	loading: boolean;
	data: SvelteMap<Uuid, Metadata>;
}>({
	loading: true,
	data: new SvelteMap()
});

export async function loadAllMetadata() {
	metadataState.loading = true;
	try {
		const metadata = await Commands.get_all();

		metadataState.data = new SvelteMap<Uuid, Metadata>(
			metadata.map((metadata) => [metadata.id, metadata])
		);
	} catch (err) {
		console.error('Error loading all metadata:', err);
	} finally {
		metadataState.loading = false;
	}
}

export function upsertMetadata(metadata: Metadata) {
	metadataState.data.set(metadata.id, metadata);
}

export function removeMetadata(id: Uuid) {
	metadataState.data.delete(id);
}
