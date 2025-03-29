<script lang="ts">
	import * as Form from '$lib/components/ui/form/index.js';
	import FormLanguages from './form-languages.svelte';
	import FormBuildSystems from './form-build-systems.svelte';
	import { Input } from '$lib/components/ui/input';
	import type { FormMetadata, Metadata } from '$lib/types';
	import {
		type FormResult,
		type SuperValidated,
		setError,
		setMessage,
		superForm
	} from 'sveltekit-superforms';
	import { zodClient } from 'sveltekit-superforms/adapters';
	import { MetadataFormSchema } from '$lib/schemas/metadata';
	import Badge from '../ui/badge/badge.svelte';
	import { Button } from '../ui/button';
	import { goto } from '$app/navigation';
	import Commands from '$lib/commands';
	import { resetState } from '../../../routes/project/state.svelte';
	import Textarea from '../ui/textarea/textarea.svelte';
	import FormCategories from './form-categories.svelte';
	import FormIde from './form-ide.svelte';
	import { open } from '@tauri-apps/plugin-dialog';
	import { ArrowLeft } from 'lucide-svelte';
	import Separator from '../ui/separator/separator.svelte';

	interface Props {
		metadata?: Metadata;
		form: SuperValidated<FormMetadata>;
	}

	let { metadata, form: formProp }: Props = $props();

	console.log('FormProp:', formProp);
	const form = superForm(formProp, {
		SPA: true,
		validators: zodClient(MetadataFormSchema),
		dataType: 'json',
		onUpdate: async ({ form }) => {
			console.log('FormValid:', form.valid);
			if (form.valid) {
				if (metadata) {
					await Commands.update_metadata(metadata.id, form.data)
						.then((metadata) => {
							resetState();
							history.back();
						})
						.catch((err) => {
							setError(form, err);
						});
				} else {
					await Commands.create_metadata(form.data)
						.then((id) => {
							resetState();
							setTimeout(() => goto(`/project`), 0);
						})
						.catch((err) => {
							setError(form, err);
						});
				}
			} else {
				console.log('Helloo I am under the water');
			}
		}
	});

	const { form: formData, enhance } = form;

	const formTitle = metadata === undefined ? 'Create a Project' : 'Edit Project';

	async function pickDirectory() {
		let dir = await open({
			multiple: false,
			directory: true
		});

		formData.update((data) => {
			data.directory = dir || '';

			return data;
		});
	}

	function goBack() {
		history.back();
	}
</script>

<div class="flex w-full justify-between">
	<Button onclick={goBack} variant="link">
		<ArrowLeft />
		Back
	</Button>
	<h1 class="text-3xl font-bold">{formTitle}</h1>
	<p></p>
</div>
<div class="w-full flex justify-center">
	<form class="w-[80%]" method="POST" use:enhance>
		<Form.Field {form} name="directory">
			<Form.Control>
				{#snippet children({ props })}
					<Form.Label>Location</Form.Label>
					<div class="flex gap-5">
						<Input {...props} bind:value={$formData.directory} />
						<Button onclick={pickDirectory} variant="secondary">Pick</Button>
					</div>
				{/snippet}
			</Form.Control>
			<Form.FieldErrors />
			<Form.Description>Where the manifest file will be placed</Form.Description>
		</Form.Field>
		<div class="divider"></div>

		<Form.Field {form} name="title">
			<Form.Control>
				{#snippet children({ props })}
					<Form.Label>Title</Form.Label>
					<Input {...props} bind:value={$formData.title} />
				{/snippet}
			</Form.Control>
			<Form.FieldErrors />
			<Form.Description></Form.Description>
		</Form.Field>
		<Separator class="m-2" />

		<Form.Field {form} name="description">
			<Form.Control>
				{#snippet children({ props })}
					<Form.Label class="flex flex-row"
						><p>Description&nbsp;</p>
						<p class="opacity-60">(Optional)</p></Form.Label
					>
					<Textarea {...props} bind:value={$formData.description} />
				{/snippet}
			</Form.Control>
			<Form.FieldErrors />
			<Form.Description></Form.Description>
		</Form.Field>

		<div class="divider"></div>
		<FormCategories {form} />

		<div class="divider"></div>
		<FormLanguages {form} />
		<br />

		<FormBuildSystems {form} />
		<br />

		<div class="divider"></div>
		<FormIde {form} />
		<br />

		<Form.Field {form} name="repositoryUrl">
			<Form.Control>
				{#snippet children({ props })}
					<Form.Label class="flex flex-row"
						><p>Repository Url&nbsp;</p>
						<p class="opacity-60">(Optional)</p></Form.Label
					>
					<Input {...props} bind:value={$formData.repositoryUrl} />
				{/snippet}
			</Form.Control>
			<Form.FieldErrors />
		</Form.Field>

		<div class="divider"></div>
		<Form.Button class="w-full bg-primary text-primary-content">Submit</Form.Button>
	</form>
</div>
