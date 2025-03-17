import { z } from 'zod';
import { CategorySchema } from './category';
import { LanguageSchema } from './language';
import { BuildSystemSchema } from './buildSystem';
import { IdeSchema } from './ide';
import type { FormMetadata, Metadata } from '$lib/types';

export const UuidSchema = z.string().uuid();

export const MetadataBackendSchema = z.object({
	id: z.string().uuid(),
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

export const MetadataFormSchema = z.object({
	title: z.string(),
	description: z.string().optional(),
	categories: z.array(CategorySchema),
	languages: z.array(LanguageSchema),
	buildSystems: z.array(BuildSystemSchema),
	preferredIde: z.optional(IdeSchema),
	repositoryUrl: z.string().optional(),
});

export function parseMetadata(data: unknown): Metadata {
	return MetadataSchema.parse(data);
}

export function parseFormData(metadata: Metadata): FormMetadata {
	return {
		title: metadata.title,
		description: metadata.description,
		categories: metadata.categories,
		languages: metadata.languages,
		buildSystems: metadata.buildSystems,
		preferredIde: metadata.preferredIde,
		repositoryUrl: metadata.repositoryUrl
	}
}

export function parseMetadataArray(data: unknown): Metadata[] {
	if (!Array.isArray(data)) {
		throw new Error('Expected array');
	}

	return data.map((item) => parseMetadata(item));
}
