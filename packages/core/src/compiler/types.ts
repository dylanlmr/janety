export interface CompileResult {
	success: boolean;
	output?: string;
	type_errors: string[];
	parse_errors: string[];
}
