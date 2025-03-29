import { superValidate } from 'sveltekit-superforms';
import type { PageLoad } from '../$types';
import { zod } from 'sveltekit-superforms/adapters';
import { MetadataFormSchema } from '$lib/schemas/metadata';

export const load: PageLoad = async () => {
	return {
		form: await superValidate(zod(MetadataFormSchema))
	};
};
