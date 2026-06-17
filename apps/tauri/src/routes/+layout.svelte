<script lang="ts">
	import ThemeWrapper from '@ui/components/theme/ThemeWrapper.svelte';
	import './layout.css';

	let { children } = $props();
</script>

<ThemeWrapper>
	{@render children()}
</ThemeWrapper>
