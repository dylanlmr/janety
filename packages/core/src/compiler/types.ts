export interface JanetyResult {
	success: boolean;
	phase: 'parsing' | 'typecheck' | 'compile' | 'runtime';
	output?: string;
	console_output: string;
	errors: string[];
}

export interface JanetyCompiler {
	compile_code(code: string): Promise<JanetyResult>;
	run_code(code: string): Promise<JanetyResult>;
	compile_and_run_code(code: string): Promise<JanetyResult>;
}
