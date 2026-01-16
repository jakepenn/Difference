<script lang="ts">
	import './layout.css';
	import { onMount } from 'svelte';
	import { handleKeyboard } from '$lib/keyboard';

	let { children } = $props();

	onMount(() => {
		window.addEventListener('keydown', handleKeyboard);
		return () => window.removeEventListener('keydown', handleKeyboard);
	});
</script>

{@render children()}
