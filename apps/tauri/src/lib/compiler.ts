import type { JanetyCompiler } from '#core/compiler/types';
import { invoke } from '@tauri-apps/api/core';

export const compiler: JanetyCompiler = {
	async compile(code) {
		return await invoke('compile_code', { code });
	}
};
