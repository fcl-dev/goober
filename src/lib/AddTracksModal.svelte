<!-- Open the modal using ID.showModal() method -->
<script lang="ts">
	import { convertFileSrc } from '@tauri-apps/api/tauri';
	import FaPlus from 'svelte-icons/fa/FaPlus.svelte';
	import type { Player } from './player';

	export let playlistName: string;
	export let player: ReturnType<typeof Player>;
	export let allPlaylist: Goober.Playlist;
	export let tracks: Goober.Track[];
	export let currentPlaylist: Goober.Playlist;

	let selectedTracks: Goober.Track[] = [];

	const trackClick = (track: Goober.Track) => {
		selectedTracks.push(track);
	};

	function addTracks() {
		const addModal = <HTMLFormElement>document.querySelector('#add_modal');

		addModal.showModal();
	}

	function done() {
		for (let track of selectedTracks) {
			currentPlaylist.content[0].tracks.push(track);
		}

		tracks = currentPlaylist.content.flatMap((a) => a.tracks);

		$player.tracks = tracks;

		const storagePlaylists = localStorage.getItem('playlists');

		if (!storagePlaylists) return;

		const parsedPlaylists: Goober.Playlist[] = JSON.parse(storagePlaylists);

		let i = parsedPlaylists.findIndex((p) => p.name === playlistName);

		parsedPlaylists[i] = currentPlaylist;

		localStorage.setItem('playlists', JSON.stringify(parsedPlaylists));

		// lol
		currentPlaylist = currentPlaylist;

		selectedTracks = [];
	}
</script>

<button class="btn" on:click={addTracks}>
	<div class="w-3 h-3"><FaPlus /></div>
</button>
<dialog id="add_modal" class="modal">
	<form method="dialog" class="modal-box">
		<div class="flex flex-row gap-2">
			<h3 class="font-bold text-lg">Add track</h3>
		</div>

		<div class="mt-2">
			{#each allPlaylist.content as album}
				<div class="flex flex-1">
					{#if album.cover !== ''}
						<img src={convertFileSrc(album.cover)} class="h-28 w-30 transform-gpu" />
					{/if}
					<div class="flex flex-col ml-2 w-full">
						<div
							class="flex justify-between hover:cursor-pointer hover:text-gray-400 transition-all"
							on:click={() => selectedTracks.push(...album.tracks)}
						>
							<h1 class="mt-2 text-3xl">
								{album.name}
							</h1>
							{#if album.year != 0}
								<h1 class="mt-2 text-3xl mr-5">{album.year}</h1>
							{/if}
						</div>

						<h2>{album.artist}</h2>

						<div class="mt-2">
							{#each album.tracks as track}
								<!-- svelte-ignore a11y-click-events-have-key-events -->
								<!--this differs from the regular Album Track, it doesn't have the track class so I don't have to do more
								work on top of document.getElementsByClassName to distinguish between AddTracksModal and plain Player.-->

								<div class="flex flex-row gap-2">
									<div
										class="flex hover:cursor-pointer hover:text-gray-400 transition-all gap-1"
										on:click={(e) => trackClick(track)}
									>
										{#if track.track !== ''}
											<span>
												{track.track}.
											</span>
										{/if}
										<span>
											{track.title}
										</span>
									</div>
								</div>
							{/each}
						</div>
					</div>
				</div>
			{/each}
		</div>

		<div class="modal-action sticky bottom-0 right-0">
			<button class="btn btn-info" on:click={() => (selectedTracks = [])}>Cancel</button>
			<button class="btn btn-info" on:click={done}>Done</button>
		</div>
	</form>
</dialog>
