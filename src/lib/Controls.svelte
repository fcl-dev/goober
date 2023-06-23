<script lang="ts">
	import FaPlay from 'svelte-icons/fa/FaPlay.svelte';
	import FaFastForward from 'svelte-icons/fa/FaFastForward.svelte';
	import FaFastBackward from 'svelte-icons/fa/FaFastBackward.svelte';
	import FaStop from 'svelte-icons/fa/FaStop.svelte';
	import FaPause from 'svelte-icons/fa/FaPause.svelte';
	import MdShuffle from 'svelte-icons/md/MdShuffle.svelte';
	import FaVolumeUp from 'svelte-icons/fa/FaVolumeUp.svelte';
	import FaVolumeDown from 'svelte-icons/fa/FaVolumeDown.svelte';
	import FaVolumeMute from 'svelte-icons/fa/FaVolumeMute.svelte';

	import { invoke } from '@tauri-apps/api/tauri';
	import type { Player } from './player';

	export let player: ReturnType<typeof Player>;

	let volume = 100;
	let shuffling = false;

	const stop = async () => await player.stop();

	const play = () => {
		if ($player.paused) {
			player.resume();
			return;
		}

		player.pause();
	};

	const fastForward = async () => {
		await player.playNext();
	};

	const fastBackward = async () => {
		await player.playPrevious();
	};

	const shuffle = async () => {
		shuffling = !shuffling;

		$player.shuffling = shuffling;
	};
</script>

<div class="flex">
	<div class="flex m-auto gap-0.5">
		<div
			class="h-6 w-6 border-2 border-gray-500 rounded-full bg-gray-800 text-gray-300 p-0.5 hover:cursor-pointer"
			on:click={fastBackward}
		>
			<FaFastBackward />
		</div>

		<div
			class="flex items-center justify-center h-6 w-6 border-2 border-gray-500 rounded-full bg-gray-800 text-gray-300 p-0.5 hover:cursor-pointer"
			on:click={stop}
		>
			<!-- svelte-ignore a11y-click-events-have-key-events -->
			<div class="h-3">
				<FaStop />
			</div>
		</div>

		<div
			class="flex items-center justify-center h-6 w-6 border-2 border-gray-500 rounded-full bg-gray-800 text-gray-300 p-0.5 hover:cursor-pointer"
			on:click={play}
		>
			{#if $player.playing}
				<!-- svelte-ignore a11y-click-events-have-key-events -->
				<div class="h-3">
					<FaPause />
				</div>
			{:else}
				<div class="h-3">
					<FaPlay />
				</div>
			{/if}
		</div>

		<div
			class="h-6 w-6 border-2 border-gray-500 rounded-full bg-gray-800 text-gray-300 p-0.5 hover:cursor-pointer"
			on:click={fastForward}
		>
			<FaFastForward />
		</div>
		<div
			class="h-6 w-6 border-2 border-gray-500 rounded-full bg-gray-800 hover:cursor-pointer {shuffling
				? 'text-blue-400'
				: 'text-gray-300'}"
			on:click={shuffle}
		>
			<MdShuffle />
		</div>
	</div>
	<div class="flex gap-2 absolute right-0 mt-1.5">
		<div class="flex h-4 w-4">
			{#if volume == 0}
				<FaVolumeMute />
			{/if}
			{#if volume > 0 && volume <= 50}
				<FaVolumeDown />
			{/if}
			{#if volume > 50}
				<FaVolumeUp />
			{/if}
		</div>

		<input
			type="range"
			class="accent-blue-400 mr-2"
			bind:value={volume}
			on:input={(_) => {
				$player.volume = volume / 100;

				invoke('volume', {
					volume: volume / 100
				});
			}}
		/>
	</div>
</div>
