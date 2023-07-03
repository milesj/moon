// Automatically generated by schematic. DO NOT MODIFY!

/* eslint-disable */

export interface PartialDenoConfig {
	/** @default 'deps.ts' */
	depsFile?: string | null;
	lockfile?: boolean | null;
}

export type NodeProjectAliasFormat = 'name-and-scope' | 'name-only';

export type NodeVersionFormat =
	| 'file'
	| 'link'
	| 'star'
	| 'version'
	| 'version-caret'
	| 'version-tilde'
	| 'workspace'
	| 'workspace-caret'
	| 'workspace-tilde';

export interface PartialNpmConfig {
	version?: string | null;
}

export type NodePackageManager = 'npm' | 'pnpm' | 'yarn';

export interface PartialPnpmConfig {
	version?: string | null;
}

export type NodeVersionManager = 'nodenv' | 'nvm';

export interface PartialYarnConfig {
	plugins?: string[] | null;
	version?: string | null;
}

export interface PartialNodeConfig {
	/** @default true */
	addEnginesConstraint?: boolean | null;
	aliasPackageNames?: NodeProjectAliasFormat | null;
	binExecArgs?: string[] | null;
	/** @default true */
	dedupeOnLockfileChange?: boolean | null;
	dependencyVersionFormat?: NodeVersionFormat | null;
	inferTasksFromScripts?: boolean | null;
	npm?: PartialNpmConfig | null;
	packageManager?: NodePackageManager | null;
	pnpm?: PartialPnpmConfig | null;
	/** @default true */
	syncProjectWorkspaceDependencies?: boolean | null;
	syncVersionManagerConfig?: NodeVersionManager | null;
	version?: string | null;
	yarn?: PartialYarnConfig | null;
}

export interface PartialRustConfig {
	bins?: string[] | null;
	syncToolchainConfig?: boolean | null;
	version?: string | null;
}

export interface PartialTypeScriptConfig {
	/** @default true */
	createMissingConfig?: boolean | null;
	/** @default 'tsconfig.json' */
	projectConfigFileName?: string | null;
	/** @default 'tsconfig.json' */
	rootConfigFileName?: string | null;
	/** @default 'tsconfig.options.json' */
	rootOptionsConfigFileName?: string | null;
	routeOutDirToCache?: boolean | null;
	/** @default true */
	syncProjectReferences?: boolean | null;
	syncProjectReferencesToPaths?: boolean | null;
}

export interface PartialToolchainConfig {
	/** @default 'https://moonrepo.dev/schemas/toolchain.json' */
	$schema?: string | null;
	deno?: PartialDenoConfig | null;
	extends?: string | null;
	node?: PartialNodeConfig | null;
	rust?: PartialRustConfig | null;
	typescript?: PartialTypeScriptConfig | null;
}

export interface DenoConfig {
	/** @default 'deps.ts' */
	depsFile: string;
	lockfile: boolean;
}

export interface NpmConfig {
	version: string | null;
}

export interface PnpmConfig {
	version: string | null;
}

export interface YarnConfig {
	plugins: string[];
	version: string | null;
}

export interface NodeConfig {
	/** @default true */
	addEnginesConstraint: boolean;
	aliasPackageNames: NodeProjectAliasFormat;
	binExecArgs: string[];
	/** @default true */
	dedupeOnLockfileChange: boolean;
	dependencyVersionFormat: NodeVersionFormat;
	inferTasksFromScripts: boolean;
	npm: NpmConfig;
	packageManager: NodePackageManager;
	pnpm: PnpmConfig | null;
	/** @default true */
	syncProjectWorkspaceDependencies: boolean;
	syncVersionManagerConfig: NodeVersionManager | null;
	version: string | null;
	yarn: YarnConfig | null;
}

export interface RustConfig {
	bins: string[];
	syncToolchainConfig: boolean;
	version: string | null;
}

export interface TypeScriptConfig {
	/** @default true */
	createMissingConfig: boolean;
	/** @default 'tsconfig.json' */
	projectConfigFileName: string;
	/** @default 'tsconfig.json' */
	rootConfigFileName: string;
	/** @default 'tsconfig.options.json' */
	rootOptionsConfigFileName: string;
	routeOutDirToCache: boolean;
	/** @default true */
	syncProjectReferences: boolean;
	syncProjectReferencesToPaths: boolean;
}

export interface ToolchainConfig {
	/** @default 'https://moonrepo.dev/schemas/toolchain.json' */
	$schema: string;
	deno: DenoConfig | null;
	extends: string | null;
	node: NodeConfig | null;
	rust: RustConfig | null;
	typescript: TypeScriptConfig | null;
}
