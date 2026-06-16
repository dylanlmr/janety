<script lang="ts">
	import CounterView from '@ui/components/counter/CounterView.svelte';

	let name = $state('');
	let greetMsg = $derived(`Hello, ${name}!`);
</script>

<h1 class="bg-amber-900">Welcome to SvelteKit</h1>

<input placeholder="Enter a name..." bind:value={name} />
<p>{greetMsg}</p>

<CounterView />
