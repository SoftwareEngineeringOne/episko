import { z } from 'zod';

export const BuildSystemSchema = z.object({
	name: z.string().nonempty(),
	version: z.string().optional().nullable()
});
