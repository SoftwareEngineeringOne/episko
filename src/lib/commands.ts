import { invoke } from '@tauri-apps/api/core';
import type {
	Category,
	Filter,
	FormMetadata,
	Language,
	Metadata,
	MetadataPreview,
	PagedMetadataPreview,
	Statistic,
	Uuid
} from './types';
import {
	MetadataDtoSchema,
	parseMetadata,
	parseMetadataDco,
	parseMetadataPreviewArray
} from './schemas/metadata';
import { PagedMetadataPreviewSchema } from './schemas/pagedData';
import { parseCategoryArray } from './schemas/category';
import { parseLanguageArray } from './schemas/language';
import { parseStatistics } from './schemas/statistics';

export default {
	async init_cache(): Promise<void> {
		return invoke('init_cache');
	},

	async get_all(pageNumber: number, filter: Filter): Promise<PagedMetadataPreview> {
		let sanitizedFilter: Filter = {
			query: filter.query === '' ? null : filter.query,
			category: filter.category === '' ? null : filter.category,
			language: filter.language === '' ? null : filter.language
		};

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

	async get_statistics(): Promise<Statistic> {
		return invoke('get_statistics').then((data) => parseStatistics(data));
	},

	async create_metadata(created: FormMetadata): Promise<Uuid> {
		return invoke('create_metadata', { created: parseMetadataDco(created) });
	},

	async update_metadata(id: Uuid, updated: FormMetadata): Promise<Metadata> {
		return invoke('update_metadata', { id: id, updated: parseMetadataDco(updated) }).then((data) =>
			parseMetadata(data)
		);
	},

	async delete_metadata(metadata: Metadata): Promise<void> {
		// as this is the only place where this transformation is needed
		// it can reside here for now
		let metadataDto = {
			build_systems: metadata.buildSystems,
			preferred_ide: metadata.preferredIde,
			repository_url: metadata.repositoryUrl,
			...metadata
		};
		return invoke('delete_metadata', { metadata: metadataDto });
	},

	async load_from_file(path: string): Promise<Uuid> {
		return invoke('load_from_file', { path: path });
	},

	async load_from_directory(path: string): Promise<number> {
		return invoke('load_from_directory', { path: path });
	}
};
