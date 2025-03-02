import type { ColumnDef } from "@tanstack/table-core";

// This type will be replaced by metadata type inheritance
export type Metadata = {
	id: string;
	title: string;
	languageName: string;
	updated: string;
};

export const columns: ColumnDef<Metadata>[] = [
	{
		accessorKey: "title",
		header: "Title",
	},
	{
		accessorKey: "languageName",
		header: "Programming Language",
	},
	{
		accessorKey: "updated",
		header: "Last updated",
	},
];