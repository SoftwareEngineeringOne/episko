<script lang="ts">
	import * as Form from '$lib/components/ui/form/index.js';
	import { Input } from '$lib/components/ui/input';
	import { Button } from '$lib/components/ui/button';
	import type { FormMetadata, BuildSystem } from '$lib/types';
	import type { SuperForm } from 'sveltekit-superforms/client';
	import { CirclePlus, Trash2 } from 'lucide-svelte';

	interface Props {
		form: SuperForm<FormMetadata>;
	}

	let { form }: Props = $props();

	const { form: formData } = form;

	let newBuildSystem: BuildSystem = $state({ name: '', version: null });

	function addBuildSystem() {
		if (newBuildSystem.name !== '') {
			let data = $formData;
			data.buildSystems.push(newBuildSystem);
			newBuildSystem = { name: '', version: null };
			formData.set(data);
		}
	}

	function removeBuildSystem(index: number) {
		return () => {
			let data = $formData;
			data.buildSystems.splice(index, 1);
			formData.set(data);
		};
	}
</script>

<div class="flex flex-row mb-1">
	<h2 class="text-sm">Build Systems&nbsp;</h2>
	<p class="text-sm opacity-60">(Optional)</p>
</div>
{#each $formData.buildSystems as _, i}
	<Form.Field {form} name="buildSystems">
		<Form.Control>
			{#snippet children({ props })}
				<div class="flex gap-4">
					<Input {...props} bind:value={$formData.buildSystems[i].name} />
					<Input {...props} bind:value={$formData.buildSystems[i].version} />
					<Button variant="destructive" onclick={removeBuildSystem(i)}><Trash2 /></Button>
				</div>
			{/snippet}
		</Form.Control>
		<Form.FieldErrors />
	</Form.Field>
{/each}
<Form.Field {form} name="buildSystems">
	<Form.Control>
		{#snippet children({ props })}
			<div class="flex gap-4">
				<Input {...props} bind:value={newBuildSystem.name} />
				<Input {...props} bind:value={newBuildSystem.version} />
				<Button variant="secondary" onclick={addBuildSystem}><CirclePlus /></Button>
			</div>
		{/snippet}
	</Form.Control>
	<Form.FieldErrors />
</Form.Field>
