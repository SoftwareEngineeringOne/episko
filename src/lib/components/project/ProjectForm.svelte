<script lang="ts">
	import { Input } from '$lib/components/ui/input';
	import { Button } from '$lib/components/ui/button';
	import { Label } from '$lib/components/ui/label';
	import { Card, CardContent, CardHeader, CardTitle } from '$lib/components/ui/card';
	import { Tabs, TabsList, TabsTrigger, TabsContent } from '$lib/components/ui/tabs';
	import type { ProjectData } from '$lib/types';

	export let projectData: ProjectData;
	export let formTitle: string = 'Create New Project';
	export let onSubmit: (data: ProjectData) => void = () => {
	};

	let selectedTab: 'manual' | 'file' = 'manual';
	let fileInput: HTMLInputElement;

	function handleBrowseClick(): void {
		fileInput.click();
	}

	function handleFileChange(event: Event): void {
		const target = event.target as HTMLInputElement;
		const files = target.files;
		if (files && files.length > 0) {
			projectData.filePath = files[0].name;
		}
	}

	function handleFormSubmit(): void {
		onSubmit(projectData);
	}
</script>

<div class="container mx-auto p-4">
	<Card class="w-full max-w-3xl mx-auto shadow-md">
		<CardHeader>
			<CardTitle>{formTitle}</CardTitle>
		</CardHeader>
		<CardContent>
			<Tabs bind:value={selectedTab} class="w-full">
				<TabsList class="grid grid-cols-2 w-full">
					<TabsTrigger value="manual" class="py-2">Manual Entry</TabsTrigger>
					<TabsTrigger value="file" class="py-2">From File</TabsTrigger>
				</TabsList>

				<TabsContent value="manual">
					<div class="space-y-4 mt-4">
						<div>
							<Label for="directory">Directory</Label>
							<Input id="directory" bind:value={projectData.directory} placeholder="Enter directory"
										 class="mt-1 w-full" />
						</div>
						<div>
							<Label for="title">Title</Label>
							<Input id="title" bind:value={projectData.title} placeholder="Enter title" class="mt-1 w-full" />
						</div>
						<div>
							<Label for="categories">Categories</Label>
							<Input id="categories" bind:value={projectData.categories} placeholder="Enter categories"
										 class="mt-1 w-full" />
						</div>
						<div>
							<Label for="languages">Languages</Label>
							<Input id="languages" bind:value={projectData.languages} placeholder="Enter languages"
										 class="mt-1 w-full" />
						</div>
						<div>
							<Label for="preferredIDE">Preferred IDE</Label>
							<Input id="preferredIDE" bind:value={projectData.preferredIDE} placeholder="Enter preferred IDE"
										 class="mt-1 w-full" />
						</div>
						<div>
							<Label for="buildSystem">Build System</Label>
							<Input id="buildSystem" bind:value={projectData.buildSystem} placeholder="Enter build system"
										 class="mt-1 w-full" />
						</div>
						<div>
							<Label for="repositoryLink">Repository Link</Label>
							<Input id="repositoryLink" bind:value={projectData.repositoryLink} placeholder="Enter repository link"
										 class="mt-1 w-full" />
						</div>
					</div>
				</TabsContent>

				<TabsContent value="file">
					<div class="mt-4">
						<Label for="filePathInput">File Path</Label>
						<div class="flex mt-1">
							<Input id="filePathInput" bind:value={projectData.filePath} placeholder="Enter file path manually"
										 class="w-full" />
							<Button type="button" on:click={handleBrowseClick} class="ml-2">Browse</Button>
							<input type="file" bind:this={fileInput} on:change={handleFileChange} class="hidden" />
						</div>
					</div>
				</TabsContent>
			</Tabs>
			<Button on:click={handleFormSubmit} class="w-full bg-primary mt-6">Save</Button>
		</CardContent>
	</Card>
</div>
