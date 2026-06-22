<script lang="ts">
	import Label from '#ui/components/ui/label/label.svelte';
	import Editor from '#ui/components/editor/editor.svelte';
	import { janetDarkTheme, janetLightTheme } from '#ui/components/editor/themes/janetTheme';

	let {
		janetyText = $bindable(),
		janetText,
		isDark = false
	}: { janetyText: string; janetText: string; isDark?: boolean } = $props();

	let activeEditorTheme = $derived(isDark ? janetDarkTheme : janetLightTheme);
</script>

<div class="flex h-full w-full flex-col gap-4 md:flex-row">
	<div class="flex min-h-0 flex-1 shrink-0 flex-col">
		<Label for="janety-editor" class="mb-2 block shrink-0 text-center text-xl">Janety</Label>
		<Editor
			bind:value={janetyText}
			id="janety-editor"
			extensions={[activeEditorTheme]}
			class="flex-1 rounded-md border border-input bg-background shadow-sm transition-colors duration-300"
		/>
	</div>

	<div class="flex min-h-0 flex-1 shrink-0 flex-col">
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
