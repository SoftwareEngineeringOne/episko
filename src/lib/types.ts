import type { z } from 'zod';
import type {
	MetadataDcoSchema,
	MetadataFormSchema,
	MetadataPreviewSchema,
	MetadataSchema,
	UuidSchema
} from './schemas/metadata';
import type { PagedMetadataPreviewSchema } from './schemas/pagedData.ts';
import type { LanguageSchema } from './schemas/language';
import type { CategorySchema } from './schemas/category';
import type { BuildSystemSchema } from './schemas/buildSystem';

export interface Filter {
	query: string | null,
	category: string | null,
	language: string | null,
}

export type Metadata = z.infer<typeof MetadataSchema>;

export type MetadataPreview = z.infer<typeof MetadataPreviewSchema>;

export type PagedMetadataPreview = z.infer<typeof PagedMetadataPreviewSchema>;

export type FormMetadata = z.infer<typeof MetadataFormSchema>;

export type MetadataDco = z.infer<typeof MetadataDcoSchema>;

export type Language = z.infer<typeof LanguageSchema>;

export type Category = z.infer<typeof CategorySchema>;

export type BuildSystem = z.infer<typeof BuildSystemSchema>;

export type Uuid = z.infer<typeof UuidSchema>;
