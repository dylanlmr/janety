<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import { Label } from '$lib/components/ui/label';
	import { toggleMode, mode, systemPrefersMode } from 'mode-watcher';

	import { Editor, janetLightTheme, janetDarkTheme } from '$lib/features/editor';
	import { Moon, Sun } from '@lucide/svelte';

	let janetyText = $state('');

	let janetText = $derived(!janetyText ? '' : janetyText.replace(/Janety/g, 'Janet'));

	let currentMode = $derived(mode.current ?? systemPrefersMode.current ?? 'light');
	let activeEditorTheme = $derived(currentMode === 'dark' ? janetDarkTheme : janetLightTheme);
</script>

<div
	class="container mx-auto flex h-dvh flex-col gap-4 bg-background p-4 text-foreground transition-colors duration-300"
>
	<div class="relative flex flex-row items-center justify-center">
		<h1 class="flex-1 text-center text-4xl font-bold">Janety</h1>
		<Button onclick={toggleMode} variant="outline" size="icon" class="absolute right-0">
			<Sun
				class="h-[1.2rem] w-[1.2rem] scale-100 rotate-0 transition-all! dark:scale-0 dark:-rotate-90"
			/>
			<Moon
				class="absolute h-[1.2rem] w-[1.2rem] scale-0 rotate-90 transition-all! dark:scale-100 dark:rotate-0"
			/>
			<span class="sr-only">Toggle theme</span>
		</Button>
	</div>

	<div class="flex flex-1 flex-col gap-4 md:flex-row">
		<div class="flex min-h-full shrink-0 flex-col md:min-h-0 md:flex-1">
			<Label for="janety-editor" class="mb-2 block shrink-0 text-center text-xl">Janety</Label>

			<Editor
				bind:value={janetyText}
				id="janety-editor"
				extensions={[activeEditorTheme]}
				class="flex-1 rounded-md border border-input bg-background shadow-sm transition-colors duration-300"
			/>
		</div>

		<div class="flex min-h-full pb-4 md:pb-0 shrink-0 flex-col md:min-h-0 md:flex-1">
			<Label for="janet-editor" class="mb-2 block shrink-0 text-center text-xl">Janet</Label>

			<Editor
				value={janetText}
				readonly={true}
				id="janet-editor"
				extensions={[activeEditorTheme]}
				class="flex-1 rounded-md border border-input bg-muted/50 shadow-inner transition-colors duration-300"
			/>
		</div>
	</div>
</div>
