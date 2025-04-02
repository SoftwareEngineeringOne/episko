<script lang="ts">
	import { goto } from '$app/navigation';
	import Commands from '$lib/commands';
	import type { Filter, Uuid } from '$lib/types';
	import { FolderGit } from 'lucide-svelte';
	import { statisticsState } from './statistics/state.svelte';

	const previewsPromise = Commands.get_all(1, {} as Filter).then((page) => page.data.slice(0, 4));

	const statistics = $derived(statisticsState.statistic);
</script>

<div class="flex flex-col align-center h-full justify-start gap-4">
	<div class="hero bg-base-100 mx-4 w-[90%] rounded-box shadow-md">
		<div class="hero-content text-center">
			<div class="max-w-md">
				<h1 class="text-5xl font-bold">Welcome back!</h1>
				<p class="py-6">Your advertisement could be here</p>
				<a href="/project" class="btn btn-primary">Your projects!</a>
			</div>
		</div>
	</div>

	<div class="flex mx-4 w-[90%] items-between gap-4">
		{#await previewsPromise}
			<div class="size-full flex justify-center items-center">
				<span class="loading loading-spinner loading-xl"></span>
			</div>
		{:then projects}
			<ul class="list bg-base-100 w-[50%] rounded-box shadow-md">
				<li class="p-4 pb-2 text-xs opacity-60 tracking-wide">Check these out!</li>
				{#each projects as project}
					<a class=" hover:cursor-pointer" href={`/project?id=${project.id}`}>
						<li class="list-row">
							<div><FolderGit /></div>
							<div>
								<div>{project.title}</div>
								<div class="text-xs opacity-60">{project.description}</div>
							</div>
						</li>
					</a>
				{/each}
			</ul>
		{/await}
		<div class="bg-base-100 w-[50%] rounded-box shadow-md">
			<div class="stats">
				<div class="stat">
					<div class="stat-title">Total Projects</div>
					{#if statistics}
						<div class="stat-value">{statistics.numberOfProjects}</div>
					{:else}
						<div class="stat-value">..</div>
					{/if}
					<div class="stat-desc">That's quite impressive!</div>
				</div>
			</div>
			<div class="stats">
				<div class="stat">
					<div class="stat-title">Different Languages</div>
					{#if statistics}
						<div class="stat-value">{statistics.numberOfLanguages}</div>
					{:else}
						<div class="stat-value">..</div>
					{/if}
					<div class="stat-desc">You appear to be quite the polyglot</div>
				</div>
			</div>
		</div>
	</div>
</div>

<!--<div
	class="min-h-screen flex items-center justify-center p-4 bg-linear-to-br from-indigo-500 via-purple-500 to-pink-500 transition-all duration-500"
>
	<Card.Root
		class="w-[380px] backdrop-blur-xs bg-white/90 dark:bg-gray-800/90 shadow-xl hover:shadow-2xl transition-all duration-300 rounded-xl [--ring:267_100%_60%]"
	>
		<Card.Header class="space-y-2">
			<Card.Title
				class="text-3xl font-bold text-center bg-linear-to-r from-indigo-500 to-pink-500 bg-clip-text text-transparent"
			>
				{#if gs.greet}
					<p>{gs.greet}</p>
					<small class="text-sm">(from Rust side)</small>
				{:else}
					<p>Hello World!</p>
				{/if}
			</Card.Title>
		</Card.Header>
		<Card.Content class="p-6">
			<form {onsubmit} class="space-y-6">
				{#if !gs.greet}
					<Input
						type="text"
						placeholder="Enter your username"
						bind:value={gs.name}
						class="w-full px-4 py-2 rounded-lg outline-hidden border border-gray-200 dark:border-gray-700 focus:ring-2 focus:ring-purple-500 focus-visible:ring-purple-500 focus-visible:ring-offset-2 transition-all duration-200"
					/>
				{/if}
				{#if gs.name && !gs.greet}
					<Button
						type="submit"
						class="w-full bg-linear-to-r from-indigo-500 to-pink-500 hover:opacity-90 transition-opacity duration-200"
					>
						Say Hello
					</Button>
				{:else if gs.greet}
					<Button
						{onclick}
						class="w-full bg-linear-to-r from-indigo-500 to-pink-500 hover:opacity-90 transition-opacity duration-200"
						>Reset</Button
					>
				{/if}
			</form>
		</Card.Content>
	</Card.Root>
</div>-->
