import { MetadataFormSchema, parseMetadata } from '$lib/schemas/metadata';
import Commands from '$lib/commands';
import type { LayoutLoad } from '../../$types';
import { superValidate } from 'sveltekit-superforms';
import { zod } from 'sveltekit-superforms/adapters';
import { parseFormData } from '$lib/schemas/metadata';

export const load: LayoutLoad = async ({ params }) => {
	if (!params.id) {
		throw Error('no id given');
	}

	let project = await Commands.get_with_id(params.id);

	let parsedProject = parseFormData(project);
	console.log('Project:', project);
	console.log('Parsed Project:', parsedProject);
	return {
		project,
		form: await superValidate(parseFormData(project), zod(MetadataFormSchema))
	};
};
