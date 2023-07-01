<script lang="ts">
	import { convertFileSrc } from '@tauri-apps/api/tauri';
	import type { Player } from './player';
	import _ from 'lodash';

	export let player: ReturnType<typeof Player>;
	export let album: Goober.Album;
	export let allTracks: Goober.Track[];
	export let currentPlaylist: Goober.Playlist;

	let name = album.name;
	let artist = album.artist;
	let cover = album.cover;
	let year = album.year;
	let tracks = album.tracks;

	$: name = album.name;
	$: artist = album.artist;
	$: cover = album.cover;
	$: year = album.year;
	$: tracks = album.tracks;

	async function playTrack(track: Goober.Track, event: MouseEvent) {
		const targetHTML = event.target as HTMLElement;
		const target = targetHTML.children.length > 0 ? targetHTML : targetHTML.parentElement;

		player.tryClearInterval();

		player.updateElement(target!);

		// ({"x": "y"}) === ({"x": "y"}) // false
		// ({"x": "y"}) == ({"x": "y"}) // false
		// thanks, javascript!
		let i = allTracks.findIndex((t) => _.isEqual(t, track));

		console.log(i);
		$player.i = i;
		$player.element = target!;

		// TODO: maybe find a better way to do this?
		// this ensures that every single time you play the track
		// goober sets player.tracks, because it's possible that
		// player.default was called (on stop) and it reset the tracks.
		$player.tracks = allTracks;

		player.play(track);
	}
</script>

<div class="flex flex-1">
	{#if cover !== ''}
		<img src={convertFileSrc(cover)} class="h-28 w-30" />
	{/if}
	<div class="flex flex-col ml-2 w-full">
		<div class="flex justify-between">
			<h1 class="mt-2 text-3xl">{name}</h1>
			{#if year != 0}
				<h1 class="mt-2 text-3xl mr-5">{year}</h1>
			{/if}
		</div>

		<h2>{artist}</h2>

		<div class="mt-2">
			{#each tracks as track, i}
				<!-- svelte-ignore a11y-click-events-have-key-events -->
				<div
					class="flex hover:cursor-pointer hover:text-gray-400 transition-all track gap-1"
					on:click={(event) => playTrack(track, event)}
				>
					{#if currentPlaylist.name === '<all>'}
						{#if track.track !== ''}
							<span>
								{track.track}.
							</span>
						{/if}
					{:else}
						{#if track.trackAlbum.cover !== ''}
							<img src={convertFileSrc(track.trackAlbum.cover)} class="h-6 w-6" />
						{/if}
						<span>
							{String(i + 1).padStart(2, '0')}.
						</span>
					{/if}

					<span>
						{track.title}
					</span>
				</div>
			{/each}
		</div>
	</div>
</div>
