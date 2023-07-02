<!-- Open the modal using ID.showModal() method -->
<script lang="ts">
	import { convertFileSrc } from '@tauri-apps/api/tauri';
	import FaPlus from 'svelte-icons/fa/FaPlus.svelte';
	import type { Player } from './player';
	import FaArrowUp from 'svelte-icons/fa/FaArrowUp.svelte';
	import FaEraser from 'svelte-icons/fa/FaEraser.svelte';

	export let playlistName: string;
	export let player: ReturnType<typeof Player>;
	export let allPlaylist: Goober.Playlist;
	export let tracks: Goober.Track[];
	export let currentPlaylist: Goober.Playlist;

	let selectedTracks: Goober.Track[] = [];

	const trackClick = (track: Goober.Track) => {
		selectedTracks.push(track);

		selectedTracks = selectedTracks;
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

		close();
	}

	function close() {
		selectedTracks = [];

		(<HTMLFormElement>document.querySelector('#add_modal')).close();
	}

	function removeLast() {
		selectedTracks.pop();

		selectedTracks = selectedTracks;
	}

	function emptySelected() {
		selectedTracks = [];
	}
</script>

<button class="btn" on:click={addTracks}>
	<div class="w-3 h-3"><FaPlus /></div>
</button>

<dialog id="add_modal" class="modal">
	<form class="modal-box w-screen max-w-7xl">
		<div class="flex flex-row gap-2">
			<h3 class="font-bold text-lg">Add tracks</h3>
		</div>

		<div class="flex flex-row w-full">
			<div class="mt-2 w-full border-r-2 border-r-gray-600">
				{#each allPlaylist.content as album}
					<div class="flex flex-1">
						{#if album.cover !== ''}
							<img src={convertFileSrc(album.cover)} class="h-28 w-30 transform-gpu" />
						{/if}
						<div class="flex flex-col ml-2 w-full">
							<div
								class="flex justify-between hover:cursor-pointer hover:text-gray-400 transition-all"
								on:click={() => {
									selectedTracks.push(...album.tracks);

									selectedTracks = selectedTracks;
								}}
							>
								<h1 class="mt-2 text-3xl break-words">
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

			<div class="flex flex-col sticky self-start top-0 w-1/2" id="selected">
				<div class="flex justify-center text-2xl">
					<h1>Selected tracks</h1>
				</div>

				{#each selectedTracks as track}
					<h1>{track.title}</h1>
				{/each}
			</div>
		</div>

		<div class="modal-action sticky bottom-0 right-0">
			<button class="btn btn-info" on:click={emptySelected}>
				<div class="h-6 w-6">
					<FaEraser />
				</div>
			</button>
			<button class="btn btn-info" on:click={removeLast}
				><div class="w-6 h-6"><FaArrowUp /></div></button
			>
			<button class="btn btn-info" on:click={close}>Cancel</button>
			<button class="btn btn-info" on:click={done}>Done</button>
		</div>
	</form>
</dialog>
