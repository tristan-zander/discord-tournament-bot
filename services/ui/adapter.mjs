// import { parse, stringify } from 'yaml';
import path from 'path';
import { rollup } from 'rollup';
import { readFile, writeFile } from 'fs/promises';
import { nodeResolve } from '@rollup/plugin-node-resolve';
import commonjs from '@rollup/plugin-commonjs';

const baseConfig = {
	alerts: [{ rule: 'DEPLOYMENT_FAILED' }, { rule: 'DOMAIN_FAILED' }]
};

/**
 * @typedef {Object} DigitalOceanOptions
 * @property {string} [outDir]
 * @property {string} appName
 * @property {string} domain
 * @property {string} region
 */

/**
 * @param {DigitalOceanOptions} options
 */
export const adapter = (options) => {
	if (!options.appName) {
		throw new Error('appName must be provided');
	}

	if (!options.domain) {
		throw new Error('domain must be provided');
	}

	if (!options.region) {
		throw new Error('region must be provided');
	}

	const outDir = path.resolve(options.outDir || 'build');

	/**
	 * @type {import('@sveltejs/kit').Adapter}
	 */
	const builder = {
		name: 'digitalocean',
		adapt: async function (builder) {
			builder.rimraf(outDir);

			const tmp = builder.getBuildDirectory('digitalocean');
			builder.writeClient(path.join(tmp, 'client'));
			builder.writePrerendered(path.join(tmp, 'prerendered'));
			builder.writeServer(path.join(tmp, 'server'));

			const entrypoint = await readFile('./entry.js', 'utf-8');

			await writeFile(path.join(tmp, 'entry.js'), entrypoint.replace(/%SERVER%/g, './server'));

			const build = await rollup({
				output: {
					file: path.join(outDir, 'server.js'),
					format: 'cjs',
					compact: true,
					inlineDynamicImports: true,
					esModule: false
				},
				input: path.join(tmp, 'entry.js'),
				plugins: [commonjs(), nodeResolve()]
			});

			await build.write({
				file: path.join(outDir, 'server.js'),
				format: 'cjs',
				compact: true,
				inlineDynamicImports: true,
				esModule: false
			});
		}
	};

	return builder;
};
