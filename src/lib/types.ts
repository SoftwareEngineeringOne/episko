import type { z } from 'zod';
import type { MetadataSchema, UuidSchema } from './schemas/metadata';
import type { LanguageSchema } from './schemas/language';
import type { CategorySchema } from './schemas/category';
import type { BuildSystemSchema } from './schemas/buildSystem';

export type Metadata = z.infer<typeof MetadataSchema>;

export type Language = z.infer<typeof LanguageSchema>;

export type Category = z.infer<typeof CategorySchema>;

export type BuildSystem = z.infer<typeof BuildSystemSchema>;

export type Uuid = z.infer<typeof UuidSchema>;
