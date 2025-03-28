<script lang="ts">
	import { type FormMetadata } from '$lib/types';
	import type { SuperForm } from 'sveltekit-superforms/client';
	import { Badge } from '../ui/badge';
	import Button from '../ui/button/button.svelte';
	import { Input } from '../ui/input';

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

Categories
{#each $formData.categories as _, i}
	<Badge
		>{$formData.categories[i].name}
		<Button variant="ghost" onclick={removeCategory(i)}>X</Button></Badge
	>&nbsp;
{/each}
<Input bind:value={newCategory} />
<Button variant="default" onclick={addCategory}>Add</Button>
