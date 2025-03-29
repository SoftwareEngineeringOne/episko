import type { Statistic } from '$lib/types';
import { z } from 'zod';

export const StatisticDtoSchema = z.object({
	projects_by_language: z.record(z.number()),
	projects_by_ide: z.record(z.number()),
	projects_by_category: z.record(z.number()),
	projects_by_build_system: z.record(z.number()),
	number_of_projects: z.number(),
	number_of_languages: z.number()
});

export const StatisticSchema = StatisticDtoSchema.transform((data) => ({
	projectsByLanguage: Object.entries(data.projects_by_language).map(([language, projects]) => ({
		language,
		projects
	})),
	projectsByIde: Object.entries(data.projects_by_ide).map(([ide, projects]) => ({
		ide,
		projects
	})),
	projectsByCategory: Object.entries(data.projects_by_category).map(([category, projects]) => ({
		category,
		projects
	})),
	projectsByBuildSystem: Object.entries(data.projects_by_build_system).map(
		([buildSystem, projects]) => ({
			buildSystem,
			projects
		})
	),
	numberOfProjects: data.number_of_projects,
	numberOfLanguages: data.number_of_languages
}));

export function parseStatistics(data: unknown): Statistic {
	try {
		return StatisticSchema.parse(data);
	} catch (err) {
		console.error('failed to parse statistic', err);
		throw err;
	}
}
