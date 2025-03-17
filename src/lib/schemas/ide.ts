import { z } from 'zod';

export const IdeSchema = z.object({
	name: z.string()
});

export type Ide = z.infer<typeof IdeSchema>;
