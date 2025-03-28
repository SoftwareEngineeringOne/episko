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
			formData.update((data) => {
				data.categories.splice(index, 1);

				return data;
			});
		};
	}

	function addCategory() {
		formData.update((data) => {
			data.categories.push({ name: newCategory });
			newCategory = '';

			return data;
		});
	}
</script>

<h2 class="text-sm mb-1">Categories</h2>
<div class="flex gap-2 mb-2">
	{#each $formData.categories as _, i}
		<Badge class="hover:cursor-pointer hover:bg-destructive hover:text-destructive-foreground">
			<p>{$formData.categories[i].name}&nbsp;</p>
			<button onclick={removeCategory(i)}><Delete size={12} /></button>
		</Badge>
	{/each}
</div>
<div class="flex gap-5">
	<Input bind:value={newCategory} />
	<Button variant="secondary" onclick={addCategory}>Add</Button>
</div>
