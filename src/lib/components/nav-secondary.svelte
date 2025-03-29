<script lang="ts">
	import * as Sidebar from '$lib/components/ui/sidebar/index.js';
	import type { ComponentProps } from 'svelte';
	import { toast } from 'svelte-sonner';

	let {
		ref = $bindable(null),
		items,
		...restProps
	}: {
		items: {
			title: string;
			url: string;
			// This should be `Component` after lucide-svelte updates types
			// eslint-disable-next-line @typescript-eslint/no-explicit-any
			icon: any;
		}[];
	} & ComponentProps<typeof Sidebar.Group> = $props();

	function disabledClick() {
		toast.warning('This function is not implemented yet!');
	}
</script>

<Sidebar.Group bind:ref {...restProps}>
	<Sidebar.GroupContent>
		<Sidebar.Menu>
			{#each items as item (item.title)}
				<Sidebar.MenuItem>
					<Sidebar.MenuButton size="sm">
						{#snippet child({ props })}
							<button onclick={disabledClick} {...props}>
								<item.icon />
								<span>{item.title}</span>
							</button>
						{/snippet}
					</Sidebar.MenuButton>
				</Sidebar.MenuItem>
			{/each}
		</Sidebar.Menu>
	</Sidebar.GroupContent>
</Sidebar.Group>
