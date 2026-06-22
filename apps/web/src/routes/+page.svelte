<script lang="ts">
	import Workspace from '#features/workspace/workspace.svelte';
	import ThemeToggle from '#ui/components/theme-toggle.svelte';
	import { compileCode, type CompileResult } from '$lib/compiler';
	import { mode, systemPrefersMode } from 'mode-watcher';

	let isDark = $derived((mode.current ?? systemPrefersMode ?? 'light') === 'dark');

	let janetyText = $state('');
	let janetText = $state('heyyyy');

	$effect(() => {
		if (!janetyText) {
			janetText = '';
			return;
		}

		compileCode(janetyText).then((result: CompileResult) => {
			if (result.success && result.output) {
				janetText = result.output;
			} else {
				janetText = '';
				console.error('Erreurs:', result.type_errors, result.parse_errors);
			}
		});
	});
</script>

<div
	class="flex h-dvh flex-col gap-4 bg-background p-4 text-foreground transition-colors duration-300"
>
	<header class="relative flex shrink-0 flex-row items-center justify-center">
		<h1 class="flex-1 text-center text-4xl font-bold">Janety</h1>
		<ThemeToggle class="absolute right-0" />
	</header>

	<main class="flex min-h-0 flex-1 flex-col">
		<Workspace bind:janetyText {janetText} {isDark} />
	</main>
</div>
