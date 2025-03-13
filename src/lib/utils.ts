import { type ClassValue, clsx } from 'clsx';
import { twMerge } from 'tailwind-merge';
import type { Metadata } from './types';

export function cn(...inputs: ClassValue[]) {
	return twMerge(clsx(inputs));
}

export function sanity_check(a: number, b: number): number {
	return a + b;
}

export function parseMetadata(data: any): Metadata {
	return {
		id: data.id,
		title: data.title,
		description: data.description ?? undefined,
		categories: data.category,
		languages: data.language,
		buildSystems: data.build_system,
		preferredIde: data.preffered_ide ?? undefined,
		repositoryUrl: data.repository_url ?? undefined,
		created: new Date(data.created),
		updated: new Date(data.updated)
	};
}

export function flyAndScale(node: HTMLElement, { duration = 300 } = {}) {
	return {
		duration,
		css: (t: number) => `
      transform: scale(${t});
      opacity: ${t};
    `
	};
}
