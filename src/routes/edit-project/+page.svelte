<script lang="ts">
	import ProjectForm from '$lib/components/project/ProjectForm.svelte';
	import type { PageData } from './$types';
	import type { ProjectData } from '$lib/types';
	import { Button } from '$lib/components/ui/button';
	import {
		Dialog,
		DialogTrigger,
		DialogContent,
		DialogTitle,
		DialogDescription,
		DialogFooter,
		DialogClose
	} from '$lib/components/ui/dialog';

	export let data: PageData;

	let projectData = data.projectData;

	function handleEdit(submittedData: ProjectData): void {
		console.log('Saving project data:', submittedData);
		//TODO: Update logic
	}

	function handleDelete(): void {
		console.log('Deleting project');
		//TODO: Deletion logic
	}
</script>

<ProjectForm formTitle="Edit Project" {projectData} onSubmit={handleEdit} />

<div class="flex justify-end mt-4">
	<Dialog>
		<DialogTrigger asChild>
			<Button class="bg-red-500">Delete Project</Button>
		</DialogTrigger>
		<DialogContent class="max-w-md">
			<DialogTitle>Confirm Deletion</DialogTitle>
			<DialogDescription>
				Are you sure you want to delete this project? This action cannot be undone.
			</DialogDescription>
			<DialogFooter class="flex justify-end gap-2">
				<Button on:click={handleDelete} class="bg-red-500">Confirm</Button>
				<DialogClose asChild>
					<Button class="bg-gray-500">Cancel</Button>
				</DialogClose>
			</DialogFooter>
		</DialogContent>
	</Dialog>
</div>
