<script lang="ts">
	import * as Form from '$lib/components/ui/form/index.js';
	import { Input } from '$lib/components/ui/input';
	import { Button } from '$lib/components/ui/button';
	import type { FormMetadata, Language } from '$lib/types';
	import type { SuperForm } from 'sveltekit-superforms/client';
	import { CirclePlus, Trash2 } from 'lucide-svelte';

	interface Props {
		form: SuperForm<FormMetadata>;
	}

	let { form }: Props = $props();

	const { form: formData } = form;

	let newLanguage: Language = $state({ name: '', version: null });

	function addLanguage() {
		// $formData.languages.push(newLanguage);
		if (newLanguage.name !== '') {
			formData.update((data) => {
				data.languages.push(newLanguage);
				return data;
			});
			newLanguage = { name: '', version: null };
		}
	}

	function removeLanguage(index: number) {
		return () => {
			formData.update((data) => {
				data.languages.splice(index, 1);
				return data;
			});
		};
	}
</script>

<h2 class="text-sm mb-1">Languages</h2>
{#each $formData.languages as _, i}
	<Form.Field {form} name="languages">
		<Form.Control>
			{#snippet children({ props })}
				<div class="flex gap-4">
					<Input {...props} bind:value={$formData.languages[i].name} />
					<Input {...props} bind:value={$formData.languages[i].version} />
					<Button variant="destructive" onclick={removeLanguage(i)}><Trash2 /></Button>
				</div>
			{/snippet}
		</Form.Control>
		<Form.FieldErrors />
	</Form.Field>
{/each}
<Form.Field {form} name="languages">
	<Form.Control>
		{#snippet children({ props })}
			<div class="flex gap-4">
				<Input {...props} bind:value={newLanguage.name} />
				<Input {...props} bind:value={newLanguage.version} />
				<Button variant="secondary" onclick={addLanguage}><CirclePlus /></Button>
			</div>
		{/snippet}
	</Form.Control>
	<Form.FieldErrors />
</Form.Field>
