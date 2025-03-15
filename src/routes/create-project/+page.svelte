<script lang="ts">
	import { goto } from '$app/navigation';
	import ProjectForm from '$lib/components/project/ProjectForm.svelte';
	import type { Metadata } from '$lib/types';

	// let projectData: Metadata = {
	// 	directory: '',
	// 	title: '',
	// 	categories: [],
	// 	languages: [],
	// 	preferredIDE: none,
	// 	buildSystem: '',
	// 	repositoryLink: '',
	// 	filePath: ''
	// };
	//
	// function handleCreate(submittedData: ProjectData): void {
	// 	console.log('Creating project with data:', submittedData);
	// 	// TODO: Creation logic
	// }

	goto('/project/import');
</script>

<ProjectForm formTitle="Create New Project" {projectData} onSubmit={handleCreate} />
