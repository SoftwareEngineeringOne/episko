import { parseMetadata } from '$lib/schemas/metadata';
import Commands from '$lib/commands';
import type { PageLoad } from './$types';


export const load: PageLoad = ({ params }) => {
	let projectPromise = Commands.get_with_id(params.id).then((project) => {
		return parseMetadata(project);
	});

	return {
		projectPromise
	};
};
