import { type ClassValue, clsx } from 'clsx';
import { twMerge } from 'tailwind-merge';

export function cn(...inputs: ClassValue[]) {
	return twMerge(clsx(inputs));
}

export const preventDefault = <T extends Event>(fn: (e: T) => void): ((e: T) => void) => {
	return (e: T) => {
		e.preventDefault();
		fn(e);
	};
};

export function sanity_check(a: number, b: number): number {
	return a + b;
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
