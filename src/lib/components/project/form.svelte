<script lang="ts">
	import * as Form from '$lib/components/ui/form/index.js';
	import FormLanguages from './form-languages.svelte';
	import { Input } from '$lib/components/ui/input';
	import type { FormMetadata, Metadata } from '$lib/types';
	import { type SuperValidated, superForm } from 'sveltekit-superforms';
	import { zodClient } from 'sveltekit-superforms/adapters';
	import { MetadataFormSchema } from '$lib/schemas/metadata';

	interface Props {
		metadata?: Metadata;
		form: SuperValidated<FormMetadata>;
	}

	let { metadata, form: formProp }: Props = $props();

	console.log('FormProp:', formProp);
	const form = superForm(formProp, {
		SPA: true,
		validators: zodClient(MetadataFormSchema),
		dataType: 'json'
	});

	const { form: formData, enhance } = form;

	const formTitle = metadata === undefined ? 'Create a Project' : 'Edit Project';
</script>

<h1>{formTitle}</h1>
<form use:enhance>
	<Form.Field {form} name="title">
		<Form.Control>
			{#snippet children({ props })}
				<Form.Label>Title</Form.Label>
				<Input {...props} bind:value={$formData.title} />
			{/snippet}
		</Form.Control>
		<Form.Description>TBC: Title of the Project</Form.Description>
		<Form.FieldErrors />
	</Form.Field>

	<FormLanguages {form} />

	<Form.Button>Submit</Form.Button>
</form>

<!--
<div class="container mx-auto p-4">
	<Card class="w-full max-w-3xl mx-auto shadow-md">
		<CardHeader>
			<CardTitle>{formTitle}</CardTitle>
		</CardHeader>
	<CardContent>
			<Tabs value="manual" class="w-full">
				<TabsList class="grid grid-cols-2 w-full">
					<TabsTrigger value="manual" class="py-2">Manual Entry</TabsTrigger>
					<TabsTrigger value="file" class="py-2">From File</TabsTrigger>
				</TabsList>

				<TabsContent value="manual">
					<div class="space-y-4 mt-4">
						<div>
							<Label for="directory">Directory</Label>
							<Input
								id="directory"
								bind:value={projectData.directory}
								placeholder="Enter directory"
								class="mt-1 w-full"
							/>
						</div>
						<div>
							<Label for="title">Title</Label>
							<Input
								id="title"
								bind:value={projectData.title}
								placeholder="Enter title"
								class="mt-1 w-full"
							/>
						</div>
						<div>
							<Label for="categories">Categories</Label>
							<Input
								id="categories"
								bind:value={projectData.categories}
								placeholder="Enter categories"
								class="mt-1 w-full"
							/>
						</div>
						<div>
							<Label for="languages">Languages</Label>
							<Input
								id="languages"
								bind:value={projectData.languages}
								placeholder="Enter languages"
								class="mt-1 w-full"
							/>
						</div>
						<div>
							<Label for="preferredIDE">Preferred IDE</Label>
							<Input
								id="preferredIDE"
								bind:value={projectData.preferredIde}
								placeholder="Enter preferred IDE"
								class="mt-1 w-full"
							/>
						</div>
						<div>
							<Label for="buildSystem">Build System</Label>
							<Input
								id="buildSystem"
								bind:value={projectData.buildSystem}
								placeholder="Enter build system"
								class="mt-1 w-full"
							/>
						</div>
						<div>
							<Label for="repositoryLink">Repository Link</Label>
							<Input
								id="repositoryLink"
								bind:value={projectData.repositoryUrl}
								placeholder="Enter repository link"
								class="mt-1 w-full"
							/>
						</div>
					</div>
				</TabsContent>

				<TabsContent value="file">
					<div class="mt-4">
						<Label for="filePathInput">File Path</Label>
						<div class="flex mt-1">
							<Input
								id="filePathInput"
								bind:value={projectData.filePath}
								placeholder="Enter file path manually"
								class="w-full"
							/>
							<Button type="button" onclick={handleBrowseClick} class="ml-2">Browse</Button>
							<input
								type="file"
								bind:this={fileInput}
								on:change={handleFileChange}
								class="hidden"
							/>
						</div>
					</div>
				</TabsContent>
			</Tabs>
			<Button onclick={handleFormSubmit} class="w-full bg-primary mt-6">Save</Button>
		</CardContent>
	</Card>
</div>
-->
