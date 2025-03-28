<script lang="ts">
	import * as Form from '$lib/components/ui/form/index.js';
	import { Input } from '$lib/components/ui/input';
	import { Button } from '$lib/components/ui/button';
	import type { FormMetadata, BuildSystem } from '$lib/types';
	import type { SuperForm } from 'sveltekit-superforms/client';

	interface Props {
		form: SuperForm<FormMetadata>;
	}

	let { form }: Props = $props();

	const { form: formData } = form;

	let newBuildSystem: BuildSystem = $state({ name: '', version: null });

	function addBuildSystem() {
		if (newBuildSystem.name !== '') {
			formData.update((data) => {
				data.buildSystems.push(newBuildSystem);
				return data;
			});
			newBuildSystem = { name: '', version: null };
		}
	}

	function removeBuildSystem(index: number) {
		return () => {
			formData.update((data) => {
				data.buildSystems.splice(index, 1);
				return data;
			});
		};
	}
</script>

Build Systems
{#each $formData.buildSystems as _, i}
	<Form.Field {form} name="buildSystems">
		<Form.Control>
			{#snippet children({ props })}
				<div class="flex">
					<Input {...props} bind:value={$formData.buildSystems[i].name} />
					<Input {...props} bind:value={$formData.buildSystems[i].version} />
					<Button variant="destructive" onclick={removeBuildSystem(i)}>Remove</Button>
				</div>
			{/snippet}
		</Form.Control>
		<Form.FieldErrors />
	</Form.Field>
{/each}
<Form.Field {form} name="buildSystems">
	<Form.Control>
		{#snippet children({ props })}
			<div class="flex">
				<Input {...props} bind:value={newBuildSystem.name} />
				<Input {...props} bind:value={newBuildSystem.version} />
				<Button variant="default" onclick={addBuildSystem}>Add</Button>
			</div>
		{/snippet}
	</Form.Control>
	<Form.FieldErrors />
</Form.Field>
