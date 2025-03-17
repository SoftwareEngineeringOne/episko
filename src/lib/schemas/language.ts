import { z } from 'zod';

export const LanguageSchema = z.object({
	name: z.string(),
	version: z.string().optional().nullable()
});
