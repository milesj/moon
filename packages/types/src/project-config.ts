// Automatically generated by schematic. DO NOT MODIFY!

/* eslint-disable */

import type { PartialTaskConfig, PlatformType, TaskConfig } from './tasks-config';

export type DependencyScope = 'build' | 'development' | 'peer' | 'production' | 'root';

export type DependencySource = 'explicit' | 'implicit';

/** Expanded information about a project dependency. */
export interface DependencyConfig {
	/** ID of the depended on project. */
	id: string;
	/**
	 * Scope of the dependency relationship.
	 *
	 * @default 'production'
	 */
	scope: DependencyScope;
	/**
	 * Source of where the dependeny came from.
	 *
	 * @default 'explicit'
	 */
	source: DependencySource;
	/** Metadata about the source. */
	via: string | null;
}

export type ProjectDependsOn = string | DependencyConfig;

export type LanguageType =
	| 'bash'
	| 'batch'
	| 'go'
	| 'javascript'
	| 'php'
	| 'python'
	| 'ruby'
	| 'rust'
	| 'typescript'
	| 'unknown'
	| string;

export type OwnersPaths = string[] | Record<string, string[]>;

/**
 * Defines ownership of source code within the current project, by mapping
 * file paths and globs to owners. An owner is either a user, team, or group.
 */
export interface OwnersConfig {
	/**
	 * Bitbucket only. A mapping of custom groups (prefixed with `@@@`),
	 * to a list of user and normal groups.
	 */
	customGroups: Record<string, string[]>;
	/** The default owner for `paths`. */
	defaultOwner: string | null;
	/** GitLab only. Marks the code owners section as optional. */
	optional: boolean;
	/**
	 * A mapping of file paths and file globs to owners.
	 * When a list, the `defaultOwner` is the owner, and each item is a path.
	 * When an object, the key is a path, and the value is a list of owners.
	 */
	paths: OwnersPaths;
	/**
	 * Bitbucket and GitLab only. The number of approvals required for the
	 * request to be satisfied. For Bitbucket, utilizes the `Check()` condition.
	 * For GitLab, marks the code owners section as required.
	 *
	 * @default 1
	 */
	requiredApprovals: number;
}

/** Expanded information about the project. */
export interface ProjectMetadataConfig {
	/**
	 * The Slack, Discord, etc, channel to discuss the project.
	 * Must start with a `#`.
	 */
	channel: string | null;
	/** A description on what the project does, and why it exists. */
	description: string;
	/** The individual maintainers of the project. The format is unspecified. */
	maintainers: string[];
	/** A human-readable name of the project. */
	name: string | null;
	/**
	 * The owner of the project. Can be an individual, team, or
	 * organization. The format is unspecified.
	 */
	owner: string | null;
}

export type StackType = 'backend' | 'frontend' | 'infrastructure' | 'systems' | 'unknown';

/** Overrides top-level toolchain settings. */
export interface ProjectToolchainCommonToolConfig {
	/** Version of the tool this project will use. */
	version: string | null;
}

/** Overrides top-level `typescript` settings. */
export interface ProjectToolchainTypeScriptConfig {
	/** Disables all TypeScript functionality for this project. */
	disabled: boolean;
	/** Appends sources of project reference to `include` in `tsconfig.json`. */
	includeProjectReferenceSources: boolean | null;
	/** Appends shared types to `include` in `tsconfig.json`. */
	includeSharedTypes: boolean | null;
	/** Updates and routes `outDir` in `tsconfig.json` to moon's cache. */
	routeOutDirToCache: boolean | null;
	/** Syncs all project dependencies as `references` in `tsconfig.json`. */
	syncProjectReferences: boolean | null;
	/** Syncs all project dependencies as `paths` in `tsconfig.json`. */
	syncProjectReferencesToPaths: boolean | null;
}

/** Overrides top-level toolchain settings, scoped to this project. */
export interface ProjectToolchainConfig {
	/** Overrides `bun` settings. */
	bun: ProjectToolchainCommonToolConfig | null;
	/** Overrides `deno` settings. */
	deno: ProjectToolchainCommonToolConfig | null;
	/** Overrides `node` settings. */
	node: ProjectToolchainCommonToolConfig | null;
	/** Overrides `rust` settings. */
	rust: ProjectToolchainCommonToolConfig | null;
	/** Overrides `typescript` settings. */
	typescript: ProjectToolchainTypeScriptConfig | null;
}

export type ProjectType =
	| 'application'
	| 'automation'
	| 'configuration'
	| 'library'
	| 'scaffolding'
	| 'tool'
	| 'unknown';

/** Controls how tasks are inherited. */
export interface ProjectWorkspaceInheritedTasksConfig {
	/** Excludes inheriting tasks by ID. */
	exclude: string[];
	/**
	 * Only inherits tasks by ID, and ignores the rest.
	 * When not defined, inherits all matching tasks.
	 * When an empty list, inherits no tasks.
	 */
	include: string[] | null;
	/** Renames inherited tasks to a new ID. */
	rename: Record<string, string>;
}

/** Overrides top-level workspace settings, scoped to this project. */
export interface ProjectWorkspaceConfig {
	/** Controls how tasks are inherited. */
	inheritedTasks: ProjectWorkspaceInheritedTasksConfig;
}

/**
 * Configures information and tasks for a project.
 * Docs: https://moonrepo.dev/docs/config/project
 */
export interface ProjectConfig {
	/** @default 'https://moonrepo.dev/schemas/project.json' */
	$schema: string;
	/** Other projects that this project depends on. */
	dependsOn: ProjectDependsOn[];
	/**
	 * A mapping of environment variables that will be set for
	 * all tasks within the project.
	 */
	env: Record<string, string>;
	/**
	 * A mapping of group IDs to a list of file paths, globs, and
	 * environment variables, that can be referenced from tasks.
	 */
	fileGroups: Record<string, string[]>;
	/**
	 * Overrides the ID within the project graph, as defined in
	 * the workspace `projects` setting.
	 */
	id: string | null;
	/**
	 * The primary programming language of the project.
	 *
	 * @default 'unknown'
	 */
	language: LanguageType;
	/**
	 * Defines ownership of source code within the current project, by mapping
	 * file paths and globs to owners. An owner is either a user, team, or group.
	 */
	owners: OwnersConfig;
	/**
	 * The default platform for all tasks within the project,
	 * if their platform is unknown.
	 *
	 * @default 'unknown'
	 */
	platform: PlatformType | null;
	/** Expanded information about the project. */
	project: ProjectMetadataConfig | null;
	/**
	 * The technology stack of the project, for categorizing.
	 *
	 * @default 'unknown'
	 */
	stack: StackType;
	/**
	 * A list of tags that this project blongs to, for categorizing,
	 * boundary enforcement, and task inheritance.
	 */
	tags: string[];
	/** A mapping of tasks by ID to parameters required for running the task. */
	tasks: Record<string, TaskConfig>;
	/** Overrides top-level toolchain settings, scoped to this project. */
	toolchain: ProjectToolchainConfig;
	/**
	 * The type of project.
	 *
	 * @default 'unknown'
	 */
	type: ProjectType;
	/** Overrides top-level workspace settings, scoped to this project. */
	workspace: ProjectWorkspaceConfig;
}

/** Expanded information about a project dependency. */
export interface PartialDependencyConfig {
	/** ID of the depended on project. */
	id?: string | null;
	/**
	 * Scope of the dependency relationship.
	 *
	 * @default 'production'
	 */
	scope?: DependencyScope | null;
	/**
	 * Source of where the dependeny came from.
	 *
	 * @default 'explicit'
	 */
	source?: DependencySource | null;
	/** Metadata about the source. */
	via?: string | null;
}

export type PartialProjectDependsOn = string | PartialDependencyConfig;

export type PartialOwnersPaths = string[] | Record<string, string[]>;

/**
 * Defines ownership of source code within the current project, by mapping
 * file paths and globs to owners. An owner is either a user, team, or group.
 */
export interface PartialOwnersConfig {
	/**
	 * Bitbucket only. A mapping of custom groups (prefixed with `@@@`),
	 * to a list of user and normal groups.
	 */
	customGroups?: Record<string, string[]> | null;
	/** The default owner for `paths`. */
	defaultOwner?: string | null;
	/** GitLab only. Marks the code owners section as optional. */
	optional?: boolean | null;
	/**
	 * A mapping of file paths and file globs to owners.
	 * When a list, the `defaultOwner` is the owner, and each item is a path.
	 * When an object, the key is a path, and the value is a list of owners.
	 */
	paths?: PartialOwnersPaths | null;
	/**
	 * Bitbucket and GitLab only. The number of approvals required for the
	 * request to be satisfied. For Bitbucket, utilizes the `Check()` condition.
	 * For GitLab, marks the code owners section as required.
	 *
	 * @default 1
	 */
	requiredApprovals?: number | null;
}

/** Expanded information about the project. */
export interface PartialProjectMetadataConfig {
	/**
	 * The Slack, Discord, etc, channel to discuss the project.
	 * Must start with a `#`.
	 */
	channel?: string | null;
	/** A description on what the project does, and why it exists. */
	description?: string | null;
	/** The individual maintainers of the project. The format is unspecified. */
	maintainers?: string[] | null;
	/** A human-readable name of the project. */
	name?: string | null;
	/**
	 * The owner of the project. Can be an individual, team, or
	 * organization. The format is unspecified.
	 */
	owner?: string | null;
}

/** Overrides top-level toolchain settings. */
export interface PartialProjectToolchainCommonToolConfig {
	/** Version of the tool this project will use. */
	version?: string | null;
}

/** Overrides top-level `typescript` settings. */
export interface PartialProjectToolchainTypeScriptConfig {
	/** Disables all TypeScript functionality for this project. */
	disabled?: boolean | null;
	/** Appends sources of project reference to `include` in `tsconfig.json`. */
	includeProjectReferenceSources?: boolean | null;
	/** Appends shared types to `include` in `tsconfig.json`. */
	includeSharedTypes?: boolean | null;
	/** Updates and routes `outDir` in `tsconfig.json` to moon's cache. */
	routeOutDirToCache?: boolean | null;
	/** Syncs all project dependencies as `references` in `tsconfig.json`. */
	syncProjectReferences?: boolean | null;
	/** Syncs all project dependencies as `paths` in `tsconfig.json`. */
	syncProjectReferencesToPaths?: boolean | null;
}

/** Overrides top-level toolchain settings, scoped to this project. */
export interface PartialProjectToolchainConfig {
	/** Overrides `bun` settings. */
	bun?: PartialProjectToolchainCommonToolConfig | null;
	/** Overrides `deno` settings. */
	deno?: PartialProjectToolchainCommonToolConfig | null;
	/** Overrides `node` settings. */
	node?: PartialProjectToolchainCommonToolConfig | null;
	/** Overrides `rust` settings. */
	rust?: PartialProjectToolchainCommonToolConfig | null;
	/** Overrides `typescript` settings. */
	typescript?: PartialProjectToolchainTypeScriptConfig | null;
}

/** Controls how tasks are inherited. */
export interface PartialProjectWorkspaceInheritedTasksConfig {
	/** Excludes inheriting tasks by ID. */
	exclude?: string[] | null;
	/**
	 * Only inherits tasks by ID, and ignores the rest.
	 * When not defined, inherits all matching tasks.
	 * When an empty list, inherits no tasks.
	 */
	include?: string[] | null;
	/** Renames inherited tasks to a new ID. */
	rename?: Record<string, string> | null;
}

/** Overrides top-level workspace settings, scoped to this project. */
export interface PartialProjectWorkspaceConfig {
	/** Controls how tasks are inherited. */
	inheritedTasks?: PartialProjectWorkspaceInheritedTasksConfig | null;
}

/**
 * Configures information and tasks for a project.
 * Docs: https://moonrepo.dev/docs/config/project
 */
export interface PartialProjectConfig {
	/** @default 'https://moonrepo.dev/schemas/project.json' */
	$schema?: string | null;
	/** Other projects that this project depends on. */
	dependsOn?: PartialProjectDependsOn[] | null;
	/**
	 * A mapping of environment variables that will be set for
	 * all tasks within the project.
	 */
	env?: Record<string, string> | null;
	/**
	 * A mapping of group IDs to a list of file paths, globs, and
	 * environment variables, that can be referenced from tasks.
	 */
	fileGroups?: Record<string, string[]> | null;
	/**
	 * Overrides the ID within the project graph, as defined in
	 * the workspace `projects` setting.
	 */
	id?: string | null;
	/**
	 * The primary programming language of the project.
	 *
	 * @default 'unknown'
	 */
	language?: LanguageType | null;
	/**
	 * Defines ownership of source code within the current project, by mapping
	 * file paths and globs to owners. An owner is either a user, team, or group.
	 */
	owners?: PartialOwnersConfig | null;
	/**
	 * The default platform for all tasks within the project,
	 * if their platform is unknown.
	 *
	 * @default 'unknown'
	 */
	platform?: PlatformType | null;
	/** Expanded information about the project. */
	project?: PartialProjectMetadataConfig | null;
	/**
	 * The technology stack of the project, for categorizing.
	 *
	 * @default 'unknown'
	 */
	stack?: StackType | null;
	/**
	 * A list of tags that this project blongs to, for categorizing,
	 * boundary enforcement, and task inheritance.
	 */
	tags?: string[] | null;
	/** A mapping of tasks by ID to parameters required for running the task. */
	tasks?: Record<string, PartialTaskConfig> | null;
	/** Overrides top-level toolchain settings, scoped to this project. */
	toolchain?: PartialProjectToolchainConfig | null;
	/**
	 * The type of project.
	 *
	 * @default 'unknown'
	 */
	type?: ProjectType | null;
	/** Overrides top-level workspace settings, scoped to this project. */
	workspace?: PartialProjectWorkspaceConfig | null;
}
