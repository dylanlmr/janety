import type { CompileResult } from './compiler';
import { invoke } from '@tauri-apps/api/core';

export async function compileCode(code: string): Promise<CompileResult> {
	try {
		const result = await invoke<CompileResult>('compile_code', { code });
		return result;
	} catch (error) {
		console.error('Tauri command error:', error);
		return {
			success: false,
			type_errors: [],
			parse_errors: ["Erreur lors de l'appel à Tauri"]
		};
	}
}
