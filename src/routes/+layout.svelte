<script lang="ts">
	import '../app.css';
	import AppSidebar from '$lib/components/app-sidebar.svelte';
	import { ModeWatcher, mode } from 'mode-watcher';
	import { toggleMode } from 'mode-watcher';
	import { Button } from '$lib/components/ui/button';
	import { Separator } from '$lib/components/ui/separator/index.js';
	import * as Sidebar from '$lib/components/ui/sidebar';
	import { Moon, Sun } from 'lucide-svelte';
	import Commands from '$lib/commands';
	import { loadStatistics } from './statistics/state.svelte';
	import { onMount } from 'svelte';
	import { preloadFirstPage } from './project/state.svelte';
	import { Toaster } from '$lib/components/ui/sonner';

	let { children } = $props();

	// temporary solution, should be done in background
	let initPromise = Commands.init_cache();

	onMount(async () => {
		await loadStatistics();
		await preloadFirstPage();
	});

	let lightMode = $derived($mode === 'light');
</script>

<ModeWatcher />
<Sidebar.Provider>
	<AppSidebar />
	<Sidebar.Inset>
		<header class="flex h-16 shrink-0 items-center justify-between gap-2">
			<div class="flex items-center gap-2 px-4">
				<Sidebar.Trigger class="-ml-1" />
				<Separator orientation="vertical" class="mr-2 h-4" />
			</div>
			<div class="flex items-center gap-2 px-4">
				<Button onclick={toggleMode} variant="outline" size="icon">
					<Sun
						class="h-[1.2rem] w-[1.2rem] rotate-0 scale-100 transition-all dark:-rotate-90 dark:scale-0"
					/>
					<Moon
						class="absolute h-[1.2rem] w-[1.2rem] rotate-90 scale-0 transition-all dark:rotate-0 dark:scale-100"
					/>
					<span class="sr-only">Toggle theme</span>
				</Button>
			</div>
			<input
				type="checkbox"
				value="lofi"
				class="toggle theme-controller hidden"
				bind:checked={lightMode}
			/>
		</header>
		<div class="flex flex-1 flex-col gap-4 p-4 pt-0">
			{#await initPromise}
				<div class="w-full h-full flex justify-center items-center flex-col">
					<h1>Loading application</h1>
					<p>This will be improved in the future, so that loading happens in the background</p>
				</div>
			{:then}
				{@render children()}
			{:catch error}
				<div class="w-full h-full flex justify-center items-center flex-col">
					<h1>Something went very wrong</h1>
					<p>{error}</p>
				</div>
			{/await}
		</div>
	</Sidebar.Inset>
</Sidebar.Provider>
<Toaster richColors />
