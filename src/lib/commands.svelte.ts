import { invoke } from '@tauri-apps/api/core';
import type { Metadata, Uuid } from './types';
import { parseMetadata, parseMetadataArray } from './schemas/metadata';

export class Commands {
	static async get_all(): Promise<Metadata[]> {
		return invoke('get_all').then((data) => parseMetadataArray(data));
	}

	static async get_with_id(id: Uuid): Promise<Metadata> {
		return invoke('get_with_id', { id: id }).then((data) => parseMetadata(data));
	}

	static async load_from_file(path: File): Promise<Uuid> {
		return invoke('load_from_file', { path: path });
	}

	static async load_from_directory(path: File): Promise<number> {
		return invoke('load_from_directory', { path: path });
	}
}
