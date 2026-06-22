import type { CompileResult } from '#core/compiler/types';
import { invoke } from '@tauri-apps/api/core';

export async function compileCode(code: string): Promise<CompileResult> {
	return await invoke('compile_code', { code });
}
