<script lang="ts" module>
	import {
		ChartArea,
		FolderOpen,
		Home,
		Import,
		Plus,
		SettingsIcon,
		SquarePen
	} from 'lucide-svelte';

	const data = {
		user: {
			name: 'shadcn',
			email: 'm@example.com',
			avatar: '/avatars/shadcn.jpg'
		},
		navMain: [
			{
				title: 'Home',
				url: '/',
				icon: Home
			},
			{
				title: 'All Projects',
				url: '/project',
				icon: FolderOpen
			},
			{
				title: 'Create project',
				url: '/project/new',
				icon: Plus
			},
			{
				title: 'Statistics',
				url: '/statistics',
				icon: ChartArea
			}
		],
		navSecondary: [
			{
				title: 'Settings',
				url: '/settings',
				icon: SettingsIcon
			}
		]
	};
</script>

<script lang="ts">
	import NavMain from '$lib/components/nav-main.svelte';
	import NavSecondary from '$lib/components/nav-secondary.svelte';
	import * as Sidebar from '$lib/components/ui/sidebar/index.js';
	import Command from 'lucide-svelte/icons/command';
	import type { ComponentProps } from 'svelte';

	let { ref = $bindable(null), ...restProps }: ComponentProps<typeof Sidebar.Root> = $props();
</script>

<Sidebar.Root bind:ref variant="inset" {...restProps}>
	<Sidebar.Header>
		<Sidebar.Menu>
			<Sidebar.MenuItem>
				<Sidebar.MenuButton size="lg">
					{#snippet child({ props })}
						<a {...props}>
							<div
								class="bg-sidebar-primary text-sidebar-primary-foreground flex aspect-square size-8 items-center justify-center rounded-lg"
							>
								<Command class="size-4" />
							</div>
							<div class="grid flex-1 text-left text-sm leading-tight">
								<span class="truncate font-semibold">Episko</span>
							</div>
						</a>
					{/snippet}
				</Sidebar.MenuButton>
			</Sidebar.MenuItem>
		</Sidebar.Menu>
	</Sidebar.Header>
	<Sidebar.Content>
		<NavMain items={data.navMain} />
	</Sidebar.Content>
	<Sidebar.Footer>
		<NavSecondary items={data.navSecondary} class="mt-auto" />
	</Sidebar.Footer>
</Sidebar.Root>
