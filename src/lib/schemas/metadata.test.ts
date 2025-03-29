import { describe, it, expect } from 'vitest';
import { parseMetadata } from './metadata'; // Adjust based on actual exports

describe('Metadata Functions', () => {
	describe('parseMetadata', () => {
		it('should parse valid metadata string', () => {
			const receivedMetadata = {
				id: '123e4567-e89b-12d3-a456-426614174000',
				title: 'Valid Title',
				directory: '/path/to/project',
				description: 'A valid description.',
				categories: [{ name: 'Category 1' }], // Example category
				languages: [{ name: 'Language 1' }], // Example language
				build_systems: [{ name: 'Build System 1' }], // Example build system
				preferred_ide: null,
				repository_url: null,
				created: '2023-01-01T00:00:00.000Z',
				updated: '2023-01-02T00:00:00.000Z'
			};

			const result = parseMetadata(receivedMetadata);

			expect(result).toEqual({
				id: '123e4567-e89b-12d3-a456-426614174000',
				title: 'Valid Title',
				directory: '/path/to/project',
				description: 'A valid description.',
				categories: [{ name: 'Category 1' }],
				languages: [{ name: 'Language 1' }],
				buildSystems: [{ name: 'Build System 1' }],
				preferredIde: undefined,
				repositoryUrl: undefined,
				created: new Date('2023-01-01T00:00:00.000Z'),
				updated: new Date('2023-01-02T00:00:00.000Z')
			});
		});

		it('should throw an error for invalid metadata string', () => {
			const invalidMetadata = {
				id: 'invalid-uuid',
				title: 'Valid Title',
				directory: '/path/to/project'
				// Missing required fields like categories, languages, etc.
			};

			expect(() => parseMetadata(invalidMetadata)).toThrow();
		});
	});
});
