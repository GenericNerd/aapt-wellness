<script lang="ts">
	import { enhance } from '$app/forms';
	import { writable } from 'svelte/store';
	import Alert from '../lib/components/Alert.svelte';
	import { AlertType } from '../lib/interfaces/alertType';
	import type { ActionData } from './$types';
	import './index.css';

	export let form: ActionData;
	const successAchieved = writable(false);

	$: successAchieved.set($successAchieved || (form !== null && form.success));
</script>

<div class="background absolute left-0 top-0 h-screen w-screen">
	<div class="background-colors h-full w-full"></div>
</div>
<div class="absolute left-0 top-0 h-screen w-screen backdrop-blur-[2px]">
	<div class="flex h-full w-full flex-col items-center justify-center">
		<img src="/img/logo.png" class="h-32 w-32" alt="AAPT Wellness Logo" />
		<h1 class="text-center text-white">AAPT Wellness</h1>
		<form
			method="POST"
			class="m-4 flex max-w-screen-sm flex-col gap-2 rounded-2xl bg-black/50 p-4 text-white"
			use:enhance
		>
			<h2 class="text-center">{$successAchieved ? 'Thank you!' : 'Interest Form'}</h2>
			{#if $successAchieved}
				<p class="text-center">
					Thanks for filling in the interest form! We'll reach out to you with any news!
				</p>
			{:else}
				<p class="text-center">
					By filling in this form, you are letting us know that you are interested in AAPT Wellness
					opening, and that you may be contacted with promotional material with details about the
					establishment if it does open.
				</p>
			{/if}
			{#if form !== null}
				<Alert
					type={form.success ? AlertType.Success : AlertType.Error}
					message={form.message}
					on:close={() => (form = null)}
				></Alert>
			{/if}{#if !$successAchieved}
				<div class="grid basis-full">
					<label for="name" class:text-red-400={form?.badData?.includes(`name`)}>Name</label>
					<input id="name" name="name" type="text" class="text-black" />
				</div>
				<div class="grid basis-full">
					<label for="email" class:text-red-400={form?.badData?.includes(`email`)}>Email</label>
					<input id="email" name="email" type="email" class="text-black" />
				</div>
				<button
					type="submit"
					class="text white mt-2 rounded-md bg-green-600 py-2 text-center text-lg transition-colors hover:bg-green-700"
					>Submit interest</button
				>
			{/if}
		</form>
	</div>
</div>
