<script lang="ts">
	import { newCounter } from './counter.svelte';
	let counter = newCounter();
</script>

<button onclick={() => counter.decrement()}>-</button>

<span class="bg-green-900">Current count: {counter.count}</span>

<button onclick={() => counter.increment()}>+</button>
