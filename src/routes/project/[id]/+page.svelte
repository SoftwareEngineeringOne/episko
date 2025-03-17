<script lang="ts">
	import { goto } from '$app/navigation';
	import { Button } from '$lib/components/ui/button';
	import type { Uuid } from '$lib/types';
	import type { PageProps } from './$types';

	let { data }: PageProps = $props();
	let project = data.project;

	function goBack() {
		history.back();
	}

	function edit(id: Uuid) {
		return () => {
			goto(`/project/${id}/edit`);
		};
	}
</script>

<div>
	<Button onclick={goBack} variant="link">Back</Button>
	<h1>{project.title}</h1>
	<p>All details can be displayed here</p>
	<Button onclick={edit(project.id)}>Edit</Button>
</div>
