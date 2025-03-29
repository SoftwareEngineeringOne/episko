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
			let data = $formData;
			data.languages.push(newLanguage);
			newLanguage = { name: '', version: null };
			formData.set(data);
		}
	}

	function removeLanguage(index: number) {
		return () => {
			let data = $formData;
			data.languages.splice(index, 1);
			formData.set(data);
		};
	}
</script>

<div class="flex flex-row mb-1">
	<h2 class="text-sm">Languages&nbsp;</h2>
	<p class="text-sm opacity-60">(Optional)</p>
</div>
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
