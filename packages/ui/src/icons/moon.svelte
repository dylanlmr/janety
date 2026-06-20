<script lang="ts">
	import type { ComponentProps } from 'svelte';
	import { cn } from '#ui/utils';
	import MoonIcon from '@lucide/svelte/icons/moon';

	type Props = ComponentProps<typeof MoonIcon>;

	let { class: className, ...restProps }: Props = $props();
</script>

<MoonIcon class={cn(className)} {...restProps} />
