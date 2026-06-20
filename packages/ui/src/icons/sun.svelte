<script lang="ts">
	import type { ComponentProps } from 'svelte';
	import { cn } from '#ui/utils';
	import SunIcon from '@lucide/svelte/icons/sun';

	type Props = ComponentProps<typeof SunIcon>;

	let { class: className, ...restProps }: Props = $props();
</script>

<SunIcon class={cn(className)} {...restProps} />
