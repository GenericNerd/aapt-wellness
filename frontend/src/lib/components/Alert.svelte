<script lang="ts">
	import { AlertType } from '$lib/interfaces/alertType';
	import { faX } from '@fortawesome/free-solid-svg-icons';
	import { createEventDispatcher } from 'svelte';
	import Fa from 'svelte-fa';

	const dispatch = createEventDispatcher();

	export let type: AlertType = AlertType.Error;
	export let message: string = 'There was an error!';
	export let details: string | null = null;
</script>

<div
	class="relative rounded-lg px-8 py-4 text-white"
	class:bg-red-600={type === AlertType.Error}
	class:bg-yellow-600={type === AlertType.Warning}
	class:bg-green-600={type === AlertType.Success}
>
	<h3>{type.toString()}</h3>
	<p>{message}</p>
	{#if details !== null}
		<p>{details}</p>
	{/if}
	<button class="absolute right-8 top-4" on:click={() => dispatch('close')}>
		<Fa icon={faX} size="xl" />
	</button>
</div>
