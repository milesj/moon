// Automatically generated by schematic. DO NOT MODIFY!

/* eslint-disable */

export type CodeownersOrderBy = 'file-source' | 'project-name';

export interface PartialCodeownersConfig {
	globalPaths?: Record<string, string[]> | null;
	orderBy?: CodeownersOrderBy | null;
	syncOnRun?: boolean | null;
}

export interface PartialConstraintsConfig {
	enforceProjectTypeRelationships?: boolean | null;
	tagRelationships?: Record<string, string[]> | null;
}

export interface PartialGeneratorConfig {
	templates?: string[] | null;
}

export type HasherOptimization = 'accuracy' | 'performance';

export type HasherWalkStrategy = 'glob' | 'vcs';

export interface PartialHasherConfig {
	batchSize?: number | null;
	optimization?: HasherOptimization | null;
	walkStrategy?: HasherWalkStrategy | null;
	warnOnMissingInputs?: boolean | null;
}

export interface PartialNotifierConfig {
	webhookUrl?: string | null;
}

export type WorkspaceProjects = string[] | Record<string, string> | {
	globs?: string[] | null;
	sources?: Record<string, string> | null;
};

export interface PartialRunnerConfig {
	archivableTargets?: string[] | null;
	cacheLifetime?: string | null;
	inheritColorsForPipedTasks?: boolean | null;
	logRunningCommand?: boolean | null;
}

export type VcsManager = 'git' | 'svn';

export type VcsProvider = 'bitbucket' | 'github' | 'gitlab' | 'other';

export interface PartialVcsConfig {
	defaultBranch?: string | null;
	manager?: VcsManager | null;
	provider?: VcsProvider | null;
	remoteCandidates?: string[] | null;
}

export interface PartialWorkspaceConfig {
	$schema?: string | null;
	codeowners?: PartialCodeownersConfig | null;
	constraints?: PartialConstraintsConfig | null;
	extends?: string | null;
	generator?: PartialGeneratorConfig | null;
	hasher?: PartialHasherConfig | null;
	notifier?: PartialNotifierConfig | null;
	projects?: WorkspaceProjects | null;
	runner?: PartialRunnerConfig | null;
	telemetry?: boolean | null;
	vcs?: PartialVcsConfig | null;
	versionConstraint?: string | null;
}

export interface CodeownersConfig {
	globalPaths: Record<string, string[]>;
	orderBy: CodeownersOrderBy;
	syncOnRun: boolean;
}

export interface ConstraintsConfig {
	enforceProjectTypeRelationships: boolean;
	tagRelationships: Record<string, string[]>;
}

export interface GeneratorConfig {
	templates: string[];
}

export interface HasherConfig {
	batchSize: number;
	optimization: HasherOptimization;
	walkStrategy: HasherWalkStrategy;
	warnOnMissingInputs: boolean;
}

export interface NotifierConfig {
	webhookUrl: string | null;
}

export interface RunnerConfig {
	archivableTargets: string[];
	cacheLifetime: string;
	inheritColorsForPipedTasks: boolean;
	logRunningCommand: boolean;
}

export interface VcsConfig {
	defaultBranch: string;
	manager: VcsManager;
	provider: VcsProvider;
	remoteCandidates: string[];
}

export interface WorkspaceConfig {
	$schema: string;
	codeowners: CodeownersConfig;
	constraints: ConstraintsConfig;
	extends: string | null;
	generator: GeneratorConfig;
	hasher: HasherConfig;
	notifier: NotifierConfig;
	projects: WorkspaceProjects;
	runner: RunnerConfig;
	telemetry: boolean;
	vcs: VcsConfig;
	versionConstraint: string | null;
}
