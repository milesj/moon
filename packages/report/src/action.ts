import type { Action, ActionStatus } from '@moonrepo/types';
import { getDurationInMillis } from './time';

export function getIconForStatus(status: ActionStatus): string {
	// Use exhaustive checks!
	// eslint-disable-next-line default-case
	switch (status) {
		case 'cached':
			return '🟪';
		case 'cached-from-remote':
			return '🟦';
		case 'failed':
		case 'failed-and-abort':
			return '🟥';
		case 'invalid':
			return '🟨';
		case 'passed':
			return '🟩';
		case 'running':
		case 'skipped':
			return '⬛️';
	}

	return '⬜️';
}

export function hasFailed(status: ActionStatus): boolean {
	return status === 'failed' || status === 'failed-and-abort';
}

export function hasPassed(status: ActionStatus): boolean {
	return status === 'passed' || status === 'cached' || status === 'cached-from-remote';
}

export function isFlaky(action: Action): boolean {
	return action.flaky || false;
}

export function isSlow(action: Action, slowThreshold: number): boolean {
	if (!action.duration) {
		return false;
	}

	const millis = getDurationInMillis(action.duration);
	const threshold = slowThreshold * 1000; // In seconds

	return millis > threshold;
}
