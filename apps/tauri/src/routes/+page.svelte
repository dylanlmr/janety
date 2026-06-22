<script lang="ts">
	import JanetyPlayground from '#features/janety-playground/janety-playground.svelte';
	import ThemeToggle from '#ui/components/theme-toggle.svelte';
	import { compiler } from '$lib/compiler';
	import { mode, systemPrefersMode } from 'mode-watcher';

	let isDark = $derived((mode.current ?? systemPrefersMode ?? 'light') === 'dark');
</script>

<div
	class="flex h-dvh flex-col gap-4 bg-background p-4 text-foreground transition-colors duration-300"
>
	<header class="relative flex shrink-0 flex-row items-center justify-center">
		<h1 class="flex-1 text-center text-4xl font-bold">Janety</h1>
		<ThemeToggle class="absolute right-0" />
	</header>

	<main class="flex min-h-0 flex-1 flex-col">
		<JanetyPlayground {compiler} {isDark} />
	</main>
</div>
