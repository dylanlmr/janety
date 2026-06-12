export interface CompileResult {
	success: boolean;
	output?: string;
	type_errors: string[];
	parse_errors: string[];
}

export async function compileCode(code: string): Promise<CompileResult> {
	const isTauri = typeof window !== 'undefined' && '__TAURI__' in window;

	if (isTauri) {
		const { compileCode: tauriCompile } = await import('./compiler.tauri');
		return tauriCompile(code);
	} else {
		const { compileCode: webCompile } = await import('./compiler.web');
		return webCompile(code);
	}
}
