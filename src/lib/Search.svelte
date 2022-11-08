<script lang="ts">
	import { listen } from '@tauri-apps/api/event';
	import { createEventDispatcher } from 'svelte';
	import { search } from '../services/searcher';

	export let service = '';

	let placeholder = 'Search...';
	let searchValue = '';

	export let isFocused = true;

	let searchBar: HTMLInputElement = null;
	const dispatch = createEventDispatcher();

	async function onInput() {
		search(service, searchValue).then((v) => {
			dispatch('newValues', {
				values: v,
			});
		});
	}

	listen('ClearSearch', (_) => {
		searchValue = '';
	});

	listen('FocusSearch', (_) => {
		isFocused = true;
		if (searchBar) {
			searchBar.focus();
			onInput().then();
		}
		if (service !== '') {
			console.log('Searching ' + service);
		}
	});
</script>

<main class="row">
	<div class="w-full flex flex-row">
		{#if service !== ''}
			<div class="flex bg-cyan-700 rounded-lg">
				<span class="text-l text-white pt-2 px-2">{service}</span>
			</div>
		{/if}
		<input
			class="grow"
			id="search"
			selected={isFocused}
			{placeholder}
			bind:this={searchBar}
			bind:value={searchValue}
			on:input={onInput}
		/>
	</div>
</main>
