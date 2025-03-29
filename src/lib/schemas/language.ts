import type { Language } from '$lib/types';
import { z } from 'zod';

export const LanguageSchema = z.object({
	name: z.string().nonempty(),
	version: z.string().optional().nullable()
});

export function parseLanguage(data: unknown): Language {
	return LanguageSchema.parse(data);
}

export function parseLanguageArray(data: unknown): Language[] {
	if (!Array.isArray(data)) {
		throw new Error('Expected array');
	}

	return data.map((item) => parseLanguage(item));
}
