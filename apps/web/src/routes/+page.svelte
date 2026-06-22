<script lang="ts">
	import JanetyPlayground from '#features/janety-playground/janety-playground.svelte';
	import ThemeToggle from '#ui/components/theme-toggle.svelte';
	import { compiler } from '$lib/compiler';
	import { mode, systemPrefersMode } from 'mode-watcher';

	let isDark = $derived((mode.current ?? systemPrefersMode ?? 'light') === 'dark');
</script>

<div
	class="relative flex h-dvh flex-col bg-background text-foreground transition-colors duration-300 selection:bg-primary/30"
>
	<div
		class="pointer-events-none absolute inset-0 z-0 bg-[radial-gradient(var(--border)_1px,transparent_1px)] bg-size-[24px_24px] opacity-40 dark:bg-[radial-gradient(var(--primary)_1px,transparent_1px)] dark:opacity-[0.03]"
	></div>

	<div
		class="pointer-events-none absolute top-0 left-0 right-0 z-0 h-[50dvh] bg-linear-to-b from-primary/5 to-transparent dark:from-primary/10"
	></div>

	<div class="relative z-10 flex h-full flex-col">
		<header
			class="flex shrink-0 items-center justify-between border-b border-border/50 bg-background/50 px-4 py-3 backdrop-blur-md md:px-6"
		>
			<div class="flex items-center gap-4">
				<h1 class="font-mono text-sm font-bold uppercase tracking-widest">
					<span class="text-foreground">Janety</span><span class="text-primary">_Playground</span>
				</h1>
			</div>

			<div class="flex items-center gap-6">
				<div class="hidden items-center gap-2 md:flex">
					<div
						class="h-1.5 w-1.5 animate-pulse rounded-full bg-primary shadow-[0_0_5px_var(--primary)]"
					></div>
					<span class="font-mono text-[10px] uppercase tracking-widest text-muted-foreground">
						Compiler Ready
					</span>
				</div>

				<div class="h-4 w-px bg-border"></div>

				<ThemeToggle />
			</div>
		</header>

		<main class="flex min-h-0 flex-1 flex-col p-4 md:p-6">
			<JanetyPlayground {compiler} {isDark} />
		</main>
	</div>
</div>
