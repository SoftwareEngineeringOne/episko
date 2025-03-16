export type Some<T> = {
	_tag: 'Some';
	value: T;
};

export type None = {
	_tag: 'None';
};

export type Option<T> = Some<T> | None;

export type Metadata = {
	// null uuid for new project (0000-0000-0000-0000-0000)
	id: Uuid;
	directory: string;
	title: string;
	description: Option<string>;
	categories: Array<Category>;
	languages: Array<Language>;
	buildSystems: Array<BuildSystem>;
	preferredIde: Option<Ide>;
	repositoryUrl: Option<string>;
	created: Date;
	updated: Date;
};

export type MetadataOverview = Pick<Metadata, 'title' | 'categories' | 'languages' | 'updated'>;

export type Category = {
	name: string;
};

export type Language = {
	name: string;
	version: Option<string>;
};

export type BuildSystem = {
	name: string;
	version: Option<string>;
};

export type Ide = {
	name: string;
};

type Uuid = string & { readonly __brand: unique symbol };
