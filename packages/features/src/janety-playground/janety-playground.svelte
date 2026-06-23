<script lang="ts">
	import type { JanetyCompiler, JanetyResult } from '#core/compiler/types';
	import Editor from '#ui/components/editor/editor.svelte';
	import { janetDarkTheme, janetLightTheme } from '#ui/components/editor/themes/janetTheme';
	import { Label } from '#ui/components/ui/label';

	let { compiler, isDark = false }: { compiler: JanetyCompiler; isDark?: boolean } = $props();

	let activeEditorTheme = $derived(isDark ? janetDarkTheme : janetLightTheme);

	let sourceCode = $state('');
	let transpiledCode = $state('');
	let result = $state<JanetyResult | null>(null);

	let hasError = $derived(result !== null && !result.success);

	let displayText = $derived.by(() => {
		if (!result) return '';

		if (!result.success) return result.errors.join('\n');

		let text = `# --- Transpiled Janet Code ---\n${transpiledCode}\n\n`;

		if (result.console_output) {
			text += `# --- Console Output ---\n${result.console_output}\n\n`;
		}

		text += `# --- Execution Return ---\n${result.output || 'nil'}`;

		return text.trim();
	});

	$effect(() => {
		if (!sourceCode) {
			transpiledCode = '';
			result = null;
			return;
		}

		compiler.compile_code(sourceCode).then((compRes) => {
			if (!compRes.success || !compRes.output) {
				transpiledCode = '';
				result = compRes;
				return;
			}

			transpiledCode = compRes.output;

			compiler.run_code(transpiledCode).then((runRes) => {
				result = runRes;
			});
		});
	});
</script>

<div class="flex h-full w-full flex-col gap-6 md:flex-row">
	<div
		class="group flex min-h-0 flex-1 flex-col overflow-hidden rounded-sm border border-border bg-background/80 shadow-sm transition-all duration-300 focus-within:border-primary"
	>
		<div
			class="flex shrink-0 items-center justify-between border-b border-border/50 bg-muted/20 px-4 py-2"
		>
			<div class="flex items-center gap-3">
				<div
					class="h-2 w-2 rounded-full bg-muted-foreground/30 group-focus-within:bg-primary"
				></div>
				<Label class="font-mono text-xs font-bold uppercase tracking-wider text-muted-foreground">
					src/main.jty
				</Label>
			</div>
			<span class="font-mono text-[10px] uppercase tracking-widest text-muted-foreground/50"
				>Input</span
			>
		</div>
		<Editor
			bind:value={sourceCode}
			extensions={[activeEditorTheme]}
			class="h-full w-full flex-1 font-mono text-sm outline-none"
		/>
	</div>

	<div
		class="flex min-h-0 flex-1 flex-col overflow-hidden rounded-sm border shadow-sm transition-colors duration-300 {hasError
			? 'border-destructive/50 bg-destructive/5'
			: 'border-border bg-muted/10'}"
	>
		<div
			class="flex shrink-0 items-center justify-between border-b border-border/50 px-4 py-2 {hasError
				? 'bg-destructive/10'
				: 'bg-muted/20'}"
		>
			<div class="flex items-center gap-3">
				<div
					class="h-2 w-2 rounded-full transition-colors duration-300 {hasError
						? 'bg-destructive'
						: 'bg-primary'}"
				></div>
				<Label
					class="font-mono text-xs font-bold uppercase tracking-wider {hasError
						? 'text-destructive'
						: 'text-muted-foreground'}"
				>
					{hasError ? 'Error Output' : 'Transpiled & Execution'}
				</Label>
			</div>
			<span class="font-mono text-[10px] uppercase tracking-widest text-muted-foreground/50"
				>Output</span
			>
		</div>

		<Editor
			value={displayText}
			readonly={true}
			extensions={[activeEditorTheme]}
			class="h-full w-full flex-1 font-mono text-sm outline-none {hasError
				? 'text-destructive'
				: ''}"
		/>
	</div>
</div>
