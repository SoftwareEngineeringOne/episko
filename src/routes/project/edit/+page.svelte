<script lang="ts">
	import type { FormMetadata, Metadata } from '$lib/types';
	import Commands from '$lib/commands';
	import { page } from '$app/state';
	import { error } from '@sveltejs/kit';
	import { superValidate, type SuperValidated } from 'sveltekit-superforms/client';
	import { MetadataFormSchema, parseFormData } from '$lib/schemas/metadata';
	import { zod } from 'sveltekit-superforms/adapters';
	import EditPage from './EditPage.svelte';

	interface Data {
		metadata: Metadata;
		form: SuperValidated<FormMetadata>;
	}
	let id = $derived(page.url.searchParams.get('id'));

	async function loadForm(): Promise<Data> {
		if (!id) {
			error(400);
		}
		let metadata = await Commands.get_with_id(id);
		return {
			metadata,
			form: await superValidate(parseFormData(metadata), zod(MetadataFormSchema))
		};
	}
</script>

{#await loadForm()}
	<div>Loading...</div>
{:then { metadata, form }}
	<EditPage {metadata} {form} />
{/await}
