<script lang="ts">
	import type { FormMetadata } from '$lib/types';
	import type { SuperForm } from 'sveltekit-superforms/client';
	import * as Form from '$lib/components/ui/form';
	import { Input } from '$lib/components/ui/input';
	import { Button } from '$lib/components/ui/button';
	import { CircleCheck, CircleX } from 'lucide-svelte';

	interface Props {
		form: SuperForm<FormMetadata>;
	}

	let { form }: Props = $props();

	const { form: formData } = form;

	function removeIde() {
		$formData.preferredIde = undefined;
	}

	function addIde() {
		$formData.preferredIde = { name: '' };
	}
</script>

<Form.Field {form} name="preferredIde.name">
	<Form.Control>
		{#snippet children({ props })}
			<Form.Label class="flex flex-row">
				<h2 class="text-sm">Preferred Ide&nbsp;</h2>
				<p class="text-sm opacity-60">(Optional)</p>
			</Form.Label>

			<div class="flex flex-row gap-4">
				{#if $formData.preferredIde}
					<Input {...props} bind:value={$formData.preferredIde!.name} />
					<Button variant="destructive" onclick={removeIde}><CircleX /></Button>
				{:else}
					<Input {...props} disabled class="bg-base-100" />
					<Button variant="secondary" onclick={addIde}><CircleCheck /></Button>
				{/if}
			</div>
		{/snippet}
	</Form.Control>
	<Form.FieldErrors />
</Form.Field>
