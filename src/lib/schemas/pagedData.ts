import { z } from 'zod';
import {
	MetadataDtoSchema,
	MetadataPreviewDtoSchema,
	parseMetadataArray,
	parseMetadataPreviewArray
} from './metadata';

export const PagedMetadataPreviewDtoSchema = z.object({
	total_size: z.number(),
	page_size: z.number(),
	page_number: z.number(),
	data: z.array(MetadataPreviewDtoSchema)
});

export const PagedMetadataPreviewSchema = PagedMetadataPreviewDtoSchema.transform((dto) => ({
	totalPages: dto.total_size,
	pageSize: dto.page_size,
	pageNumber: dto.page_number,
	data: parseMetadataPreviewArray(dto.data)
}));
