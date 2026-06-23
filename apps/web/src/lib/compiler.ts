import type { JanetyCompiler, JanetyResult } from '#core/compiler/types';
import type * as RustCompilerModule from '$lib/wasm/janety_core';
import createJanetVM from '$lib/wasm/janet_vm.js';

interface JanetModule {
	stringToNewUTF8(str: string): number;
	UTF8ToString(ptr: number): string;
	_evaluate_janet_wasm(codePtr: number): number;
	_free(ptr: number): void;
	_get_wasm_result(): number;
	_get_wasm_console(): number;
}

let rustCompiler: typeof RustCompilerModule | null = null;
let janetVMInstance: JanetModule | null = null;

async function getRustCompiler(): Promise<typeof RustCompilerModule> {
	if (!rustCompiler) {
		const mod = await import('$lib/wasm/janety_core');
		await mod.default();
		rustCompiler = mod;
	}
	return rustCompiler;
}

async function getJanetVM(): Promise<JanetModule> {
	if (!janetVMInstance) {
		janetVMInstance = (await createJanetVM()) as JanetModule;
	}
	return janetVMInstance;
}

export const compiler: JanetyCompiler = {
	async compile_code(code: string): Promise<JanetyResult> {
		const wasm = await getRustCompiler();
		const wasmResult = wasm.compile(code);

		return {
			success: wasmResult.success,
			phase: wasmResult.phase as 'parsing' | 'typecheck' | 'compile' | 'runtime',
			output: wasmResult.output,
			console_output: wasmResult.console_output,
			errors: Array.from(wasmResult.errors || [])
		};
	},

	async run_code(code: string): Promise<JanetyResult> {
		try {
			const vm = await getJanetVM();

			const codePtr = vm.stringToNewUTF8(code);

			const status = vm._evaluate_janet_wasm(codePtr);

			vm._free(codePtr);

			const outputVal = vm.UTF8ToString(vm._get_wasm_result());
			const consoleVal = vm.UTF8ToString(vm._get_wasm_console());

			return {
				success: status === 0,
				phase: 'runtime',
				output: status === 0 ? outputVal : undefined,
				console_output: consoleVal,
				errors: status !== 0 ? [outputVal] : []
			};
		} catch (error: unknown) {
			const message = error instanceof Error ? error.message : String(error);
			return {
				success: false,
				phase: 'runtime',
				output: undefined,
				console_output: '',
				errors: [message]
			};
		}
	},

	async compile_and_run_code(code: string): Promise<JanetyResult> {
		const compileResult = await this.compile_code(code);

		if (!compileResult.success || !compileResult.output) {
			return compileResult;
		}
		return await this.run_code(compileResult.output);
	}
};
