import type { Category } from '$lib/types';
import { z } from 'zod';

export const CategorySchema = z.object({
	name: z.string().nonempty()
});

export function parseCategory(data: unknown): Category {
	return CategorySchema.parse(data);
}

export function parseCategoryArray(data: unknown): Category[] {
	if (!Array.isArray(data)) {
		throw new Error('Expected array');
	}

	return data.map((item) => parseCategory(item));
}
