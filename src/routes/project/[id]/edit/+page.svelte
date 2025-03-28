<script lang="ts">
	import ProjectForm from '$lib/components/project/form.svelte';
	import type { Metadata } from '$lib/types';
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
	import type { PageProps } from './$types';

	let { data }: PageProps = $props();
	let metadata = data.project;

	console.log('Data:', data);

	function handleEdit(submittedData: Metadata): void {
		console.log('Saving project data:', submittedData);
		//TODO: Update logic
	}

	function handleDelete(): void {
		console.log('Deleting project');
		//TODO: Deletion logic
	}
</script>

<ProjectForm {metadata} form={data.form} />

<div class="flex justify-end mt-4">
	<Dialog>
		<DialogTrigger>
			<Button class="bg-red-500">Delete Project</Button>
		</DialogTrigger>
		<DialogContent class="max-w-md">
			<DialogTitle>Confirm Deletion</DialogTitle>
			<DialogDescription>
				Are you sure you want to delete this project? This action cannot be undone.
			</DialogDescription>
			<DialogFooter class="flex justify-end gap-2">
				<Button onclick={handleDelete} class="bg-red-500">Confirm</Button>
				<DialogClose>
					<Button class="bg-gray-500">Cancel</Button>
				</DialogClose>
			</DialogFooter>
		</DialogContent>
	</Dialog>
</div>
