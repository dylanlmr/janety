<script lang="ts">
	import './layout.css';
	import favicon from '$lib/assets/favicon.svg';
	import ThemeWrapper from '@ui/components/theme/ThemeWrapper.svelte';

	let { children } = $props();
</script>

<svelte:head><link rel="icon" href={favicon} /></svelte:head>
<ThemeWrapper>
	{@render children()}
</ThemeWrapper>
