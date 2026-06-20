<script lang="ts">
	import Workspace from '#features/workspace/workspace.svelte';
	import ThemeToggle from '#ui/components/theme-toggle.svelte';
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
		<Workspace {isDark} />
	</main>
</div>
