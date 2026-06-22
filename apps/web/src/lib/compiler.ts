import type { CompileResult } from '#core/compiler/types';
import type * as WasmModule from '$lib/wasm/janety_core';

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
	const wasm = await getWasmModule();
	return wasm.compile(code);
}
