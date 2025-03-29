<script lang="ts">
	import ProjectForm from '$lib/components/project/form.svelte';
	import type { FormMetadata, Metadata } from '$lib/types';
	import { Button } from '$lib/components/ui/button';
	import * as Dialog from '$lib/components/ui/dialog';
	import type { PageProps } from './$types';
	import Commands from '$lib/commands';
	import { toast } from 'svelte-sonner';
	import { goto } from '$app/navigation';
	import { page } from '$app/state';
	import { error } from '@sveltejs/kit';
	import type { SuperForm } from 'sveltekit-superforms';

	interface Props {
		metadata: Metadata;
		form: SuperForm<FormMetadata>;
	}

	let { metadata, form } = $props();

	let dialogOpen = $state(false);
	async function deleteProject() {
		Commands.delete_metadata(metadata)
			.then(() => {
				toast.success('Project deleted successfully');
				goto('/project');
			})
			.catch((err) => {
				toast.error(`Failed to delete Project: ${err}`);
				dialogOpen = false;
			});
	}
</script>

<div class="flex flex-col justify-center">
	<ProjectForm {metadata} {form} />
	<Dialog.Root bind:open={dialogOpen}>
		<Dialog.Trigger>
			<button class="btn btn-link text-destructive">Delete this Project</button>
		</Dialog.Trigger>
		<Dialog.Content class="max-w-md">
			<Dialog.Title>Confirm Deletion</Dialog.Title>
			<Dialog.Description>
				Are you sure you want to delete this project? <br />This action cannot be undone.
			</Dialog.Description>
			<Dialog.Footer class="flex justify-end gap-2">
				<Button onclick={deleteProject} variant="destructive">Confirm</Button>
				<Dialog.Close>
					<Button variant="secondary">Cancel</Button>
				</Dialog.Close>
			</Dialog.Footer>
		</Dialog.Content>
	</Dialog.Root>
</div>
