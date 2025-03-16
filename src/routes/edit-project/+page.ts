import type { PageLoad } from './$types';
import type { ProjectData } from '$lib/types';

export const load: PageLoad = async ({ fetch, params }) => {
	// TODO: replace with API call later
	const projectData: ProjectData = {
		directory: '/home/user/projects/example',
		title: 'Example Project',
		categories: 'category1, category2',
		languages: 'JavaScript, TypeScript',
		preferredIDE: 'VS Code',
		buildSystem: 'Vite',
		repositoryLink: 'https://github.com/username/example-repo',
		filePath: '/home/user/projects/example/info.txt'
	};

	return { projectData };
};
