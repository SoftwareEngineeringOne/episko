import { z } from 'zod';

export const IdeSchema = z.object({
	name: z.string().nonempty()
});

export type Ide = z.infer<typeof IdeSchema>;
