<script lang="ts">
	import { open } from '@tauri-apps/plugin-dialog';
	import type { Uuid } from '$lib/types';
	import { goto } from '$app/navigation';
	import { toast } from 'svelte-sonner';

	import { Button } from '$lib/components/ui/button';
	import * as Tabs from '$lib/components/ui/tabs';
	import Commands from '$lib/commands';

	async function pickFile(pick_dir: boolean): Promise<string | null> {
		return await open({
			multiple: false,
			directory: pick_dir
		});
	}

	async function loadFile() {
		let file = await pickFile(false);

		if (!file) {
			toast.error('Failed to pick file');
			return;
		}

		let promise = Commands.load_from_file(file);
		toast.promise(promise, {
			loading: `Loading ${file}`,
			success: (id) => {
				goto(`/project/${id}`);
				return 'Successfully loaded project!';
			},
			error: 'Failed to load project'
		});
	}

	async function loadDirectory() {
		let dir = await pickFile(true);

		if (!dir) {
			toast.error('Failed to pick directory');
			return;
		}

		let promise = Commands.load_from_directory(dir);
		toast.promise(promise, {
			loading: `Loading ${dir}`,
			success: (_) => {
				goto(`/project/`);
				return 'Successfully loaded projects!';
			},
			error: 'Failed to load projects'
		});
	}
</script>

<div class="w-full h-full flex justify-center items-center">
	<Tabs.Root value="file" class="card bg-base-200 border border-base-300 shadow-sm">
		<div class="card-body">
			<h2 class="card-title">Import Project</h2>
			<div class="divider"></div>
			<Tabs.List>
				<Tabs.Trigger value="file">Single Project</Tabs.Trigger>
				<Tabs.Trigger value="directory">From Directory</Tabs.Trigger>
			</Tabs.List>
			<div class="card-actions flex w-full justify-center">
				<Tabs.Content value="file" class="flex w-full justify-center flex-col">
					<Button onclick={loadFile} variant="default" class="hover:cursor-pointer"
						>Pick File</Button
					>
					<p class="text-sm font-semibold opacity-60 text-center">
						Choose a single Manifest to load
					</p>
				</Tabs.Content>
				<Tabs.Content value="directory" class="flex w-full justify-center flex-col">
					<Button onclick={loadDirectory} variant="default" class="hover:cursor-pointer"
						>Pick Directory</Button
					>
					<p class="text-sm font-semibold opacity-60 text-center">Choose a directory to search</p>
				</Tabs.Content>
			</div>
		</div>
	</Tabs.Root>
</div>
