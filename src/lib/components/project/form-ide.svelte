<script lang="ts">
	import type { FormMetadata } from '$lib/types';
	import type { SuperForm } from 'sveltekit-superforms/client';
	import * as Form from '$lib/components/ui/form';
	import { Input } from '$lib/components/ui/input';
	import { Button } from '$lib/components/ui/button';

	interface Props {
		form: SuperForm<FormMetadata>;
	}

	let { form }: Props = $props();

	const { form: formData } = form;

	function removeIde() {
		formData.update((data) => {
			data.preferredIde = undefined;
			return data;
		});
	}

	function addIde() {
		formData.update((data) => {
			data.preferredIde = { name: '' };
			return data;
		});
	}
</script>

{#if $formData.preferredIde}
	<Form.Field {form} name="preferredIde.name">
		<Form.Control>
			{#snippet children({ props })}
				<Form.Label>Preferred Ide</Form.Label>
				<Input {...props} bind:value={$formData.preferredIde!.name} />
			{/snippet}
		</Form.Control>
		<Form.FieldErrors />
		<Form.Description>TBC: Title of the Project</Form.Description>
	</Form.Field>
	<Button variant="destructive" onclick={removeIde}>Remove</Button>
{:else}
	<h2>Preferred Ide</h2>
	<Button variant="default" onclick={addIde}>Add</Button>
{/if}
