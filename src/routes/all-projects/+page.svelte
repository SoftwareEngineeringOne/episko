<script lang="ts">
	import DataTable from "./data-table.svelte";
	import { columns } from './columns';
	import type { Metadata } from './columns';

	//example Data - Will be imported by meta typedef
	const data: Metadata[] = [
		{
			id: "m5gr84i9",
			title: "test title",
			languageName: "python",
			updated: "27.02.2025"
		},
		{
			id: "3sd8g2s8",
			title: "test2",
			languageName: "java",
			updated: "20.02.2025"
		}
	];
</script>

<h1 class="text-2xl font-bold mb-4">All Projects</h1>

<DataTable {data} {columns} />