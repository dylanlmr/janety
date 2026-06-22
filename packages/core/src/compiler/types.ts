export interface CompileResult {
	success: boolean;
	output?: string;
	type_errors: string[];
	parse_errors: string[];
}

export interface JanetyCompiler {
	compile(code: string): Promise<CompileResult>;
}
