import { invoke } from '@tauri-apps/api/core';
import type { Category, Filter, FormMetadata, Language, Metadata, MetadataPreview, PagedMetadataPreview, Uuid } from './types';
import { parseMetadata, parseMetadataDco, parseMetadataPreviewArray } from './schemas/metadata';
import { PagedMetadataPreviewSchema } from './schemas/pagedData';
import { parseCategoryArray } from './schemas/category';
import { parseLanguageArray } from './schemas/language';

export default {
	async init_cache(): Promise<void> {
		return invoke('init_cache');
	},

	async get_all(pageNumber: number, filter: Filter): Promise<PagedMetadataPreview> {
		let sanitizedFilter: Filter = {
			query: filter.query === '' ? null : filter.query,
			category: filter.category === '' ? null : filter.category,
			language: filter.language === '' ? null : filter.language,
		}

		return invoke('get_all', { pageNumber: pageNumber, filter: sanitizedFilter }).then((data) =>
			PagedMetadataPreviewSchema.parse(data)
		);
	},

	async get_with_id(id: Uuid): Promise<Metadata> {
		return invoke('get_with_id', { id: id }).then((data) => parseMetadata(data));
	},

	async get_all_categories(): Promise<Category[]> {
		return invoke('get_all_categories').then((data) => parseCategoryArray(data));
	},

	async get_all_languages(): Promise<Language[]> {
		return invoke('get_all_languages').then((data) => parseLanguageArray(data));
	},

	async create_metadata(created: FormMetadata): Promise<Uuid> {
		return invoke('create_metadata', { created: parseMetadataDco(created) });
	},

	async update_metadata(id: Uuid, updated: FormMetadata): Promise<Metadata> {
		return invoke('update_metadata', { id: id, updated: parseMetadataDco(updated) }).then((data) =>
			parseMetadata(data)
		);
	},

	async load_from_file(path: string): Promise<Uuid> {
		return invoke('load_from_file', { path: path });
	},

	async load_from_directory(path: string): Promise<number> {
		return invoke('load_from_directory', { path: path });
	}
};
