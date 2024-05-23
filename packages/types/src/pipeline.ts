import type { Duration, Runtime } from './common';

export type ActionStatus =
	| 'cached-from-remote'
	| 'cached'
	| 'failed-and-abort'
	| 'failed'
	| 'invalid'
	| 'passed'
	| 'running'
	| 'skipped';

/** @deprecated */
export interface Attempt {
	duration: Duration | null;
	exitCode: number | null;
	finishedAt: string | null;
	index: number;
	startedAt: string;
	status: ActionStatus;
	stderr: string | null;
	stdout: string | null;
}

export type OperationType =
	| 'archive-creation'
	| 'hash-generation'
	| 'mutex-acquisition'
	| 'no-operation'
	| 'output-hydration'
	| 'task-execution';

export interface OperationOutput {
	exitCode: number | null;
	stderr: string | null;
	stdout: string | null;
}

export interface Operation {
	duration: Duration | null;
	finishedAt: string | null;
	hash: string | null;
	output: OperationOutput | null;
	startedAt: string;
	status: ActionStatus;
	type: OperationType;
}

export interface Action {
	allowFailure: boolean;
	/** @deprecated */
	attempts: Attempt[] | null;
	createdAt: string;
	duration: Duration | null;
	error: string | null;
	finishedAt: string | null;
	flaky: boolean;
	label: string;
	node: ActionNode;
	nodeIndex: number;
	operations: Operation[];
	startedAt: string | null;
	status: ActionStatus;
}

export interface TargetState {
	state: 'failed' | 'passed' | 'passthrough' | 'skipped';
	hash?: string;
}

export interface ActionContext {
	affectedOnly: boolean;
	initialTargets: string[];
	passthroughArgs: string[];
	primaryTargets: string[];
	profile: 'cpu' | 'heap' | null;
	targetStates: Record<string, TargetState>;
	touchedFiles: string[];
	workspaceRoot: string;
}

export interface RunReport {
	actions: Action[];
	context: ActionContext;
	duration: Duration;
	comparisonEstimate: {
		duration: Duration;
		gain: Duration | null;
		loss: Duration | null;
		percent: number;
		tasks: Record<
			string,
			{
				count: number;
				total: Duration;
			}
		>;
	};
	// Deprecated
	estimatedSavings?: Duration | null;
	projectedDuration?: Duration;
}

// NODES

export type ActionNode =
	| ActionNodeInstallDeps
	| ActionNodeInstallProjectDeps
	| ActionNodeRunTask
	| ActionNodeSetupTool
	| ActionNodeSyncProject
	| ActionNodeSyncWorkspace;

export interface ActionNodeInstallDeps {
	action: 'install-deps';
	params: {
		runtime: Runtime;
	};
}

export interface ActionNodeInstallProjectDeps {
	action: 'install-project-deps';
	params: {
		runtime: Runtime;
		project: string;
	};
}

export interface ActionNodeRunTask {
	action: 'run-task';
	params: {
		args: string[];
		env: Record<string, string>;
		interactive: boolean;
		persistent: boolean;
		runtime: Runtime;
		target: string;
	};
}

export interface ActionNodeSetupTool {
	action: 'setup-tool';
	params: {
		runtime: Runtime;
	};
}

export interface ActionNodeSyncProject {
	action: 'sync-project';
	params: {
		runtime: Runtime;
		project: string;
	};
}

export interface ActionNodeSyncWorkspace {
	action: 'sync-workspace';
}

// GRAPH

export interface ActionGraphNode {
	id: number;
	label: string;
}

export interface ActionGraphEdge {
	id: number;
	label: string;
	source: number;
	target: number;
}

export interface ActionGraph {
	edges: ActionGraphEdge[];
	nodes: ActionGraphNode[];
}
