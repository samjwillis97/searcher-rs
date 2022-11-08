<script lang="ts">
    import { closeWindow, getInfo } from '../../../services/commands'
    import type {InfoData} from './+page'

    export let data: InfoData

    getInfo(data.id).then((v) => {
        console.log(v)
    })

	document.onkeydown = function (event: KeyboardEvent) {
		switch (event.key) {
			case 'Escape':
				// TODO: Go back instead of closing depending on the state
				closeWindow().then();
				break;
        }
    }
</script>

<div>
    <span>Info Screen</span>
    <span>{data.id}</span>
</div>

<style>
</style>
