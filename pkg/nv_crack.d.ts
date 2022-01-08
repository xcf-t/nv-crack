/* tslint:disable */
/* eslint-disable */
/**
* @param {any} major_max
* @param {any} minor_max
* @param {any} patch_max
* @param {any} build_max
* @param {any} hash
* @returns {any}
*/
export function find_hash_origin(major_max: any, minor_max: any, patch_max: any, build_max: any, hash: any): any;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly find_hash_origin: (a: number, b: number, c: number, d: number, e: number) => number;
}

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
