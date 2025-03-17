<script lang="ts">
	import { open } from '@tauri-apps/plugin-dialog';
	import type { Uuid } from '$lib/types';
	import { goto } from '$app/navigation';
	import { toast } from 'svelte-sonner';

	import { Button } from '$lib/components/ui/button';
	import * as Tabs from '$lib/components/ui/tabs';
	import Commands from '$lib/commands';

	let loadPromise: Promise<void> | null = null;

	async function pickFile(pick_dir: boolean) {
		return await open({
			multiple: false,
			directory: pick_dir
		});
	}

	function loadFile() {
		loadPromise = pickFile(false)
			.then((path): Promise<Uuid> => {
				if (path === null) {
					throw Error('No path given');
				}
				return Commands.load_from_file(path);
			})
			.then((id) => {
				goto(`/project/${id}`);
			});
	}

	function loadDirectory() {
		loadPromise = pickFile(true)
			.then((path): Promise<number> => {
				if (path === null) {
					throw Error('No path given');
				}
				return Commands.load_from_directory(path);
			})
			.then((amount) => {
				toast(`Loaded ${amount} manifests!`);
				goto('/project');
			});
	}
</script>

<Tabs.Root value="file" class="w-[400px]">
	<Tabs.List>
		<Tabs.Trigger value="file">Load file</Tabs.Trigger>
		<Tabs.Trigger value="directory">Load directory</Tabs.Trigger>
	</Tabs.List>
	<Tabs.Content value="file">
		{#if loadPromise}
			{#await loadPromise}
				Loading file...
			{:then}
				Redirecting...
			{:catch error}
				Failed to load: ${error}
			{/await}
		{:else}
			<Button onclick={loadFile} variant="outline">Load File</Button>
		{/if}
	</Tabs.Content>
	<Tabs.Content value="directory">
		{#if loadPromise}
			{#await loadPromise}
				Loading directory...
			{:then}
				Redirecting...
			{:catch error}
				Failed to load: ${error}
			{/await}
		{:else}
			<Button onclick={loadDirectory} variant="outline">Load Directory</Button>
		{/if}
	</Tabs.Content>
</Tabs.Root>
