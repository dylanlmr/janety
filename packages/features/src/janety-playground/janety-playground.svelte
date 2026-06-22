<script lang="ts">
	import type { CompileResult, JanetyCompiler } from '#core/compiler/types';
	import Editor from '#ui/components/editor/editor.svelte';
	import { janetDarkTheme, janetLightTheme } from '#ui/components/editor/themes/janetTheme';
	import { Label } from '#ui/components/ui/label';

	let { compiler, isDark = false }: { compiler: JanetyCompiler; isDark?: boolean } = $props();

	let activeEditorTheme = $derived(isDark ? janetDarkTheme : janetLightTheme);

	let janetyText = $state('');
	let janetText = $state('');
	let hasError = $state(false);

	$effect(() => {
		if (!janetyText) {
			janetText = '';
			hasError = false;
			return;
		}

		compiler.compile(janetyText).then((result: CompileResult) => {
			if (result.success && result.output) {
				janetText = result.output;
				hasError = false;
			} else {
				janetText =
					result.parse_errors.join('\n') ||
					result.type_errors.join('\n') ||
					'Erreur de compilation.';
				hasError = true;
			}
		});
	});
</script>

<div class="flex h-full w-full flex-col gap-6 md:flex-row">
	<div
		class="group flex min-h-0 flex-1 flex-col overflow-hidden rounded-sm border border-border bg-background/80 shadow-sm backdrop-blur-md transition-all duration-300 focus-within:border-primary focus-within:ring-1 focus-within:ring-primary/50 hover:border-border/80"
	>
		<div
			class="flex shrink-0 items-center justify-between border-b border-border/50 bg-muted/20 px-4 py-2"
		>
			<div class="flex items-center gap-3">
				<div
					class="h-2 w-2 rounded-full bg-muted-foreground/30 transition-colors duration-300 group-focus-within:bg-primary group-focus-within:shadow-[0_0_8px_var(--primary)]"
				></div>
				<Label
					for="janety-editor"
					class="cursor-pointer font-mono text-xs font-bold uppercase tracking-wider text-muted-foreground group-focus-within:text-primary"
				>
					src/main.jty
				</Label>
			</div>
			<span class="font-mono text-[10px] uppercase tracking-widest text-muted-foreground/50"
				>Input</span
			>
		</div>

		<div class="flex min-h-0 flex-1 flex-col bg-transparent">
			<Editor
				bind:value={janetyText}
				id="janety-editor"
				extensions={[activeEditorTheme]}
				class="h-full w-full flex-1 font-mono text-sm outline-none"
			/>
		</div>
	</div>

	<div
		class="flex min-h-0 flex-1 flex-col overflow-hidden rounded-sm border shadow-sm backdrop-blur-md {hasError
			? 'border-destructive/50 bg-destructive/5'
			: 'border-border bg-muted/10'} transition-colors duration-300"
	>
		<div
			class="flex shrink-0 items-center justify-between border-b border-border/50 px-4 py-2 {hasError
				? 'bg-destructive/10'
				: 'bg-muted/20'}"
		>
			<div class="flex items-center gap-3">
				<div
					class="h-2 w-2 rounded-full transition-colors duration-300 {hasError
						? 'bg-destructive shadow-[0_0_8px_var(--destructive)]'
						: janetText
							? 'bg-primary'
							: 'bg-muted-foreground/30'}"
				></div>
				<Label
					for="janet-editor"
					class="font-mono text-xs font-bold uppercase tracking-wider {hasError
						? 'text-destructive'
						: 'text-muted-foreground'}"
				>
					out/main.janet
				</Label>
			</div>
			<span class="font-mono text-[10px] uppercase tracking-widest text-muted-foreground/50"
				>Output</span
			>
		</div>

		<div class="flex min-h-0 flex-1 flex-col bg-transparent opacity-90">
			<Editor
				value={janetText}
				readonly={true}
				id="janet-editor"
				extensions={[activeEditorTheme]}
				class="h-full w-full flex-1 font-mono text-sm outline-none {hasError
					? 'text-destructive'
					: ''}"
			/>
		</div>
	</div>
</div>
