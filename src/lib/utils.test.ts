import { sanity_check } from './utils';
import { expect, test } from 'vitest';

test('SanityTest', () => {
	let a = 1;
	let b = 2;

	let sum = a + b;

	expect(sanity_check(a, b)).toEqual(sum);
});
