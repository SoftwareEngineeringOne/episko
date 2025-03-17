import { invoke } from '@tauri-apps/api/core';
import type { Metadata, Uuid } from './types';
import { parseMetadata, parseMetadataArray } from './schemas/metadata';

export default {
	async get_all(): Promise<Metadata[]> {
		return invoke('get_all').then((data) => parseMetadataArray(data));
	},

	async get_with_id(id: Uuid): Promise<Metadata> {
		return invoke('get_with_id', { id: id }).then((data) => parseMetadata(data));
	},

	async load_from_file(path: string): Promise<Uuid> {
		return invoke('load_from_file', { path: path });
	},

	async load_from_directory(path: string): Promise<number> {
		return invoke('load_from_directory', { path: path });
	}
};
