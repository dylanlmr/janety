import type * as WasmModule from '$lib/wasm/janety_core';

export interface CompileResult {
	success: boolean;
	output?: string;
	type_errors: string[];
	parse_errors: string[];
}

let wasmModule: typeof WasmModule | null = null;

async function getWasmModule(): Promise<typeof WasmModule> {
	if (!wasmModule) {
		const mod = await import('$lib/wasm/janety_core');
		await mod.default();
		wasmModule = mod;
	}
	return wasmModule;
}

export async function compileCode(code: string): Promise<CompileResult> {
	console.log('Using web compiler');
	const wasm = await getWasmModule();
	return wasm.compile(code);
}
