<script lang="ts">
	import { open } from '@tauri-apps/plugin-dialog';
	import { invoke } from '@tauri-apps/api/core';
	import type { Metadata } from '$lib/types';
	import { goto } from '$app/navigation';
	import { toast } from 'svelte-sonner';

	import { Button } from '$lib/components/ui/button';
	import * as Tabs from '$lib/components/ui/tabs';

	let loadPromise: Promise<void> | null = null;

	async function pickFile(pick_dir: boolean) {
		return await open({
			multiple: false,
			directory: pick_dir
		});
	}

	function loadFile() {
		loadPromise = pickFile(false)
			.then((path): Promise<Metadata> => {
				return invoke('load_from_file', { path: path });
			})
			.then((metadata: Metadata) => {
				goto(`/project/${metadata.id}`);
			});
	}

	function loadDirectory() {
		loadPromise = pickFile(true)
			.then((path): Promise<number> => {
				return invoke('load_from_directory', { path: path });
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
			{:then file}
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
			{:then file}
				Redirecting...
			{:catch error}
				Failed to load: ${error}
			{/await}
		{:else}
			<Button onclick={loadDirectory} variant="outline">Load Directory</Button>
		{/if}
	</Tabs.Content>
</Tabs.Root>
