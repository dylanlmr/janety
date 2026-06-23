import type { JanetyCompiler, JanetyResult } from '#core/compiler/types';
import { invoke } from '@tauri-apps/api/core';

export const compiler: JanetyCompiler = {
	compile_code: async function (code: string): Promise<JanetyResult> {
		return await invoke('compile_code', { code });
	},
	run_code: async function (code: string): Promise<JanetyResult> {
		return await invoke('run_code', { code });
	},
	compile_and_run_code: async function (code: string): Promise<JanetyResult> {
		return await invoke('compile_and_run_code', { code });
	}
};
