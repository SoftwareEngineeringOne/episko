import { z } from 'zod';
import { CategorySchema } from './category';
import { LanguageSchema } from './language';
import { BuildSystemSchema } from './buildSystem';
import { IdeSchema } from './ide';
import type { Metadata } from '$lib/types';

export const MetadataBackendSchema = z.object({
	id: z.string(),
	title: z.string(),
	description: z.string().optional().nullable(),
	category: z.array(CategorySchema),
	language: z.array(LanguageSchema),
	build_system: z.array(BuildSystemSchema),
	preffered_ide: z.optional(IdeSchema).nullable(),
	repository_url: z.string().optional().nullable(),
	created: z.string(),
	updated: z.string()
});

export const MetadataSchema = MetadataBackendSchema.transform((data) => ({
	id: data.id,
	title: data.title,
	description: data.description ?? undefined,
	categories: data.category,
	languages: data.language,
	buildSystems: data.build_system,
	preferredIde: data.preffered_ide ?? undefined,
	repositoryUrl: data.repository_url ?? undefined,
	created: new Date(data.created),
	updated: new Date(data.updated)
}));

export function parseMetadata(data: unknown): Metadata {
	console.log('Parsing: ', data);
	return MetadataSchema.parse(data);
}

export function parseMetadataArray(data: unknown): Metadata[] {
	if (!Array.isArray(data)) {
		throw new Error('Expected array');
	}

	return data.map((item) => parseMetadata(item));
}
