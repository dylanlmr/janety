<script lang="ts">
	import { Button } from '#ui/components/ui/button';
	import Sun from '#ui/icons/sun.svelte';
	import Moon from '#ui/icons/moon.svelte';
	import { toggleMode } from 'mode-watcher';
	import { cn } from '#ui/utils';
	import type { ComponentProps } from 'svelte';

	type Props = ComponentProps<typeof Button>;

	let { class: className, ...restProps }: Props = $props();
</script>

<Button onclick={toggleMode} variant="outline" size="icon" class={cn(className)} {...restProps}>
	<Sun
		class="h-[1.2rem] w-[1.2rem] scale-100 rotate-0 transition-all! dark:scale-0 dark:-rotate-90"
	/>
	<Moon
		class="absolute h-[1.2rem] w-[1.2rem] scale-0 rotate-90 transition-all! dark:scale-100 dark:rotate-0"
	/>
	<span class="sr-only">Toggle theme</span>
</Button>
