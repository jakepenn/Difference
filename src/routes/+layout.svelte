<script lang="ts">
	import './layout.css';
	import { onMount } from 'svelte';
	import { handleKeyboard, refresh, refreshAll } from '$lib/keyboard';
	import { onGitChange, stopWatching } from '$lib/tauri';

	let { children } = $props();

	onMount(() => {
		window.addEventListener('keydown', handleKeyboard);

		// Listen for git changes from file watcher
		let unlisten: (() => void) | null = null;
		onGitChange((event) => {
			if (event.change_type === 'branch') {
				refreshAll();
			} else {
				refresh();
			}
		}).then((fn) => {
			unlisten = fn;
		});

		return () => {
			window.removeEventListener('keydown', handleKeyboard);
			unlisten?.();
			stopWatching();
		};
	});
</script>

{@render children()}
