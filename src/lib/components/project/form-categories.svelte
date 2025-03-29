<script lang="ts">
	import { type FormMetadata } from '$lib/types';
	import type { SuperForm } from 'sveltekit-superforms/client';
	import { Badge } from '../ui/badge';
	import Button from '../ui/button/button.svelte';
	import { Input } from '../ui/input';
	import X from '@lucide/svelte/icons/x';
	import { Delete, Trash } from 'lucide-svelte';

	interface Props {
		form: SuperForm<FormMetadata>;
	}

	let { form }: Props = $props();

	const { form: formData } = form;

	let newCategory = $state('');

	function removeCategory(index: number) {
		return () => {
			let data = $formData;
			data.categories.splice(index, 1);

			formData.set(data);
		};
	}

	function addCategory() {
		let data = $formData;
		data.categories.push({ name: newCategory });
		newCategory = '';

		formData.set(data);
	}
</script>

<div class="flex flex-row mb-1">
	<h2 class="text-sm">Categories&nbsp;</h2>
	<p class="text-sm opacity-60">(Optional)</p>
</div>
<div class="flex gap-2 mb-2">
	{#each $formData.categories as _, i}
		<button onclick={removeCategory(i)}>
			<Badge class="hover:cursor-pointer hover:bg-destructive hover:text-destructive-foreground">
				<p>{$formData.categories[i].name}&nbsp;</p>
				<Delete size={12} />
			</Badge>
		</button>
	{/each}
</div>
<div class="flex gap-5">
	<Input bind:value={newCategory} />
	<Button variant="secondary" onclick={addCategory}>Add</Button>
</div>
