import { spawn } from 'child_process';
import fs from 'fs';
import { BINARY, TARGET, getPackageFromTarget, getPath } from './helpers.mjs';

async function buildBinary() {
	// Allow arbitrary args to be passed through
	const args = process.argv.slice(2);

	// Build the binary with the provided target
	await new Promise((resolve, reject) => {
		const child = spawn('cargo', ['build', '--release', '--target', TARGET, ...args], {
			stdio: 'inherit',
			cwd: process.cwd(),
			shell: true,
		});

		child.on('error', reject);
		child.on('close', resolve);
	});

	// Copy the binary to the package
	const targetPath = getPath('target', TARGET, 'release', BINARY);
	const binaryPath = getPath('packages', getPackageFromTarget(), BINARY);

	// Copy into target core package
	await fs.promises.copyFile(targetPath, binaryPath);
	await fs.promises.chmod(binaryPath, 0o755);

	// Copy into root so that it can be uploaded as an artifact
	await fs.promises.copyFile(targetPath, getPath(BINARY));
}

buildBinary().catch((error) => {
	console.error(error);
	process.exit(1);
});
