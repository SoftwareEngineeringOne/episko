import { z } from 'zod';

export const LanguageSchema = z.object({
	name: z.string().nonempty(),
	version: z.string().optional().nullable()
});
