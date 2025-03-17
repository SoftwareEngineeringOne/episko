import { MetadataFormSchema, parseMetadata } from '$lib/schemas/metadata';
import Commands from '$lib/commands';
import type { LayoutLoad } from '../$types';
import { superValidate } from 'sveltekit-superforms';
import { zod } from 'sveltekit-superforms/adapters';
import { fail, type Actions } from '@sveltejs/kit';

export const load: LayoutLoad = async () => {
	return {
		form: await superValidate(zod(MetadataFormSchema))
	}
}

