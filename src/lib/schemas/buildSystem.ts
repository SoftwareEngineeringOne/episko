import { z } from 'zod';

export const BuildSystemSchema = z.object({
	name: z.string(),
	version: z.string().optional().nullable()
});
