<script lang="ts">
	import { openInfo, openService } from '../services/commands';

	import type { SearchResult, Shortcut } from '../services/searcher';
	import { ModifierKey } from '../services/searcher';

	export let service: string = '';
	export let item: SearchResult = null;
	export let shortcut: Shortcut = null;
	export let selected: boolean = false;

	$: htmlString = generateHtmlString(item);

	async function handleClick(event: MouseEvent) {
		//@ts-ignore
		if (!service && event.detail === 1 && event.webkitForce === 1) {
			await openService(item.id);
		} else {
			await openInfo(item.id);
		}
	}

	function generateHtmlString(item: SearchResult): String {
		if (!item) return '';
		return item.value
			.split('')
			.map((val, i) => {
				if (val === ' ') {
					val = '&nbsp;';
				}
				if (item.indices.includes(i)) {
					return `<b>${val}</b>`;
				} else {
					return val;
				}
			})
			.join('');
	}
</script>

{#if item}
	<button
		class="
        row
        w-full 
        "
		class:text-red-600={selected}
		on:click={handleClick}
	>
		<div class="w-full flex justify-between">
			<div>
				{@html htmlString}
			</div>

			<div class="flex">
				{#if shortcut?.modifier}
					<div class="border rounded-md border-stone-600 bg-stone-900 py-0 px-2 mr-1">
						{#if shortcut.modifier === ModifierKey.Cmd}
							<p>CMD</p>
						{:else if shortcut.modifier === ModifierKey.Ctrl}
							<p>CTRL</p>
						{/if}
					</div>
				{/if}
				{#if shortcut?.key}
					<div class="border rounded-md border-stone-600 bg-stone-900 py-0 px-2">
						<p>{shortcut.key}</p>
					</div>
				{/if}
			</div>
		</div>
	</button>
{/if}

<style>
</style>
