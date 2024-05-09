import { Server } from '%SERVER%/index.js';
import { manifest } from '%SERVER%/manifest.js';
import { Readable } from 'node:stream';

/**
 * https://docs.digitalocean.com/products/functions/reference/runtimes/node-js
 * @typedef {Object} DigitalOceanEvent
 * @property {DigialOceanHttpEvent} http
 */

/**
 * @typedef {Object} DigialOceanHttpEvent
 * @property {Record<string, string>} headers
 * @property {string} method
 * @property {string} path
 * @property {boolean} [isBase64Encoded]
 * @property {string} [queryString]
 * @property {string} [body]
 */

/**
 * @typedef {Object} DigitalOceanResponse
 * @property {string | object} [body]
 * @property {number} statusCode
 * @property {Record<string, string>} headers
 */

/**
 * @typedef {Object} DigitalOceanContext
 * @property {string} activationId
 * @property {string} apiHost
 * @property {string} [apiKey]
 * @property {number} deadline
 * @property {`/${string}/${string}/${string}`} functionName
 * @property {string} functionVersion
 * @property {string} namespace
 * @property {string} requestId
 */

/**
 * @param {DigitalOceanEvent} event
 * @param {DigitalOceanContext} context
 * @returns {DigitalOceanResponse} HTTP response
 */
export async function main(event, context) {
	try {
		console.debug(JSON.stringify({ event, context }));
		const server = new Server(manifest);
		await server.init({ env: process.env });

		console.debug('initialized server');

		/**
		 * @type {Buffer}
		 */
		let body;
		if (event?.http?.body) {
			console.debug(body);
			if (event?.http?.isBase64Encoded) {
				body = Buffer.from(event.http.body, 'base64');
			} else {
				body = Buffer.from(event.http.body);
			}
		}

		console.debug(JSON.stringify({ body }));

		const request = new Request(context.apiHost + event.http.path, {
			search: event.http.queryString,
			body: body ? Readable.from(body) : null,
			headers: event.http.headers,
			method: event.http.method,
		});

		console.debug(JSON.stringify({ url: request.url, method: request.method, search: request.search }));

		/**
		 * @type {Response}
		 */
		const result = await server.respond(request, {});

		console.debug(JSON.stringify({ text: result.statusText, status: result.status }));

		return {
			statusCode: result.status,
			headers: Object.fromEntries(result.headers),
			body: await result.text()
		};
	} catch (e) {
		console.error(e.toString());
		console.error(JSON.stringify(e));
		return {
			statusCode: 500
		};
	}
}
