import { parseMetadata } from '$lib/schemas/metadata';
import { error } from '@sveltejs/kit';
import type { PageLoad } from './$types';
import { invoke } from '@tauri-apps/api/core';

export const load: PageLoad = ({ params }) => {
	let id = params.id;
	console.log('Id: ', id);

	let projectPromise = invoke('get_with_id', { id: id }).then((project) => {
		return parseMetadata(project);
	});

	return {
		projectPromise
	};
};
