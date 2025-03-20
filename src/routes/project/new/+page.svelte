<script lang="ts">
	import ProjectForm from '$lib/components/project/form.svelte';
	import type { Metadata } from '$lib/types';
	import type { PageProps } from './$types';

	function handleCreate(metadata: Metadata) {
		console.log('Creating metadata: ', metadata);
	}

	let { data }: PageProps = $props();
	console.log('Data:', data);
</script>

<ProjectForm form={data.form} />
