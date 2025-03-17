import { z } from 'zod';
import { CategorySchema } from './category';
import { LanguageSchema } from './language';
import { BuildSystemSchema } from './buildSystem';
import { IdeSchema } from './ide';
import type { FormMetadata, Metadata, MetadataDco } from '$lib/types';

export const UuidSchema = z.string().uuid();

export const MetadataBackendSchema = z.object({
	id: z.string().uuid(),
	title: z.string(),
	directory: z.string(),
	description: z.string().optional().nullable(),
	categories: z.array(CategorySchema),
	languages: z.array(LanguageSchema),
	build_systems: z.array(BuildSystemSchema),
	preffered_ide: z.optional(IdeSchema).nullable(),
	repository_url: z.string().optional().nullable(),
	created: z.string(),
	updated: z.string()
});

export const MetadataSchema = MetadataBackendSchema.transform((data) => ({
	id: data.id,
	title: data.title,
	directory: data.directory,
	description: data.description ?? undefined,
	categories: data.categories,
	languages: data.languages,
	buildSystems: data.build_systems,
	preferredIde: data.preffered_ide ?? undefined,
	repositoryUrl: data.repository_url ?? undefined,
	created: new Date(data.created),
	updated: new Date(data.updated)
}));

export const MetadataFormSchema = z.object({
	title: z.string().nonempty(),
	directory: z.string().nonempty(),
	description: z.string().optional(),
	categories: z.array(CategorySchema),
	languages: z.array(LanguageSchema),
	buildSystems: z.array(BuildSystemSchema),
	preferredIde: z.optional(IdeSchema),
	repositoryUrl: z.string().optional()
});

export const MetadataDcoSchema = MetadataFormSchema.transform((data) => ({
	title: data.title,
	directory: data.directory,
	description: data.description,
	categories: data.categories,
	languages: data.languages,
	build_systems: data.buildSystems,
	preferred_ide: data.preferredIde,
	repository_url: data.repositoryUrl
}));

export function parseMetadata(data: unknown): Metadata {
	return MetadataSchema.parse(data);
}

export function parseFormData(metadata: Metadata): FormMetadata {
	return {
		title: metadata.title,
		directory: metadata.directory,
		description: metadata.description,
		categories: metadata.categories,
		languages: metadata.languages,
		buildSystems: metadata.buildSystems,
		preferredIde: metadata.preferredIde,
		repositoryUrl: metadata.repositoryUrl
	};
}

export function parseMetadataDco(data: FormMetadata): MetadataDco {
	return MetadataDcoSchema.parse(data);
}

export function parseMetadataArray(data: unknown): Metadata[] {
	if (!Array.isArray(data)) {
		throw new Error('Expected array');
	}

	return data.map((item) => parseMetadata(item));
}
