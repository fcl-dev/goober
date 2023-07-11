<script lang="ts">
	import Album from './Album.svelte';
	import Controls from './Controls.svelte';
	import { listen } from '@tauri-apps/api/event';
	import { convertFileSrc, invoke } from '@tauri-apps/api/tauri';
	import { Player } from './player';
	import { onMount } from 'svelte';
	import CreatePlaylistModal from './CreatePlaylistModal.svelte';
	import AddTracksModal from './AddTracksModal.svelte';
	import DeletePlaylistModal from './DeletePlaylistModal.svelte';
	import EmptyPlaylistModal from './EmptyPlaylistModal.svelte';
	import { register } from '@tauri-apps/api/globalShortcut';
	import { appWindow } from '@tauri-apps/api/window';

	/**
	 * All tracks in the current `Playlist`
	 */
	let tracks: Goober.Track[] = [];

	/**
	 * The music library.
	 */
	let library: Goober.Album[] = [];

	/**
	 * All playlists.
	 */
	let playlists: Goober.Playlist[] = [];

	/**
	 * The current loaded playlist.
	 */
	let currentPlaylist: Goober.Playlist = {
		name: '<all>',
		content: [],
		selected: true,
		cover: ''
	};

	/**
	 * The volume level that appears on the slider.
	 */
	let volume: number = 0;

	let player = Player();

	function progressBar(event: MouseEvent) {
		const progressBar = event.target as HTMLElement;
		const rect = progressBar.getBoundingClientRect();
		const x = event.clientX - rect.left;
		const percentage = (x / rect.width) * 100;
	}

	const onKeyDown = (e: KeyboardEvent) => {
		if (e.code !== 'Space') return;

		if ($player.playing) {
			player.pause();

			return;
		}

		player.resume();
	};

	enum Sorting {
		ByYear,
		ByArist,
		ByAlbumName
	}

	let sortingMethod = 'Year';

	function sortBy(by: Sorting) {
		switch (by) {
			case Sorting.ByYear: {
				sortingMethod = 'Year';

				currentPlaylist.content.sort((a, b) => (a.year > b.year ? 1 : -1));
				currentPlaylist = currentPlaylist;

				tracks = currentPlaylist.content.flatMap((album) => album.tracks);
				break;
			}
			case Sorting.ByArist: {
				sortingMethod = 'Artist (A-Z)';

				currentPlaylist.content.sort((a, b) => (a.artist > b.artist ? 1 : -1));
				currentPlaylist = currentPlaylist;

				tracks = currentPlaylist.content.flatMap((album) => album.tracks);
				break;
			}
			case Sorting.ByAlbumName: {
				sortingMethod = 'Album name (A-Z)';

				currentPlaylist.content.sort((a, b) => (a.name > b.name ? 1 : -1));
				currentPlaylist = currentPlaylist;

				tracks = currentPlaylist.content.flatMap((album) => album.tracks);
				break;
			}
		}
	}

	listen('library', async (event) => {
		library = (event.payload as Goober.Payload).library;

		$player.library = library;
		$player.tracks = tracks;

		localStorage.setItem('library', JSON.stringify(library));

		if (playlists[0]) {
			playlists[0].content = library;
		} else {
			let allPlaylist: Goober.Playlist = {
				name: '<all>',
				content: library,
				selected: true,
				cover: ''
			};

			playlists.push(allPlaylist);
			playlists = playlists;

			currentPlaylist = allPlaylist;
		}

		sortBy(Sorting.ByYear);

		localStorage.setItem('playlists', JSON.stringify(playlists));

		await invoke('set_presence', {
			presence: {
				state: 'Browsing',
				details: `${library.reduce((acc, e) => acc + e.tracks.length, 0)} tracks loaded`,
				largeText: `v${PKG.version}`,
				smallImage: 'browsing'
			}
		});
	});

	type Time = {
		h: number;
		m: number;
		s: number;
	};

	function toTime(totalSeconds: number): Time {
		const totalMinutes = Math.floor(totalSeconds / 60);

		const seconds = totalSeconds % 60;
		const hours = Math.floor(totalMinutes / 60);
		const minutes = totalMinutes % 60;

		return { h: hours, m: minutes, s: seconds };
	}

	function formatTime(time: Time): string {
		let h = time.h.toString();
		let m = time.m.toString();
		let s = time.s.toString();

		if (h === '0') return `${m.padStart(2, '0')}:${s.padStart(2, '0')}`;

		return `${h.padStart(2, '0')}:${m.padStart(2, '0')}:${s.padStart(2, '0')}`;
	}

	function selectPlaylist(
		e: Event & {
			currentTarget: EventTarget & HTMLSelectElement;
		}
	) {
		if (!e.target) return;

		let selectedPlaylist = playlists.find((p) => p.selected);
		let chosenPlaylist = playlists.find((p) => p.name === (<HTMLInputElement>e.target).value)!;

		if (!selectedPlaylist) return;

		currentPlaylist = chosenPlaylist;

		tracks = chosenPlaylist.content.flatMap((a) => a.tracks);

		$player.tracks = tracks;
		$player.i = -1;

		const storagePlaylists = localStorage.getItem('playlists');

		if (!storagePlaylists) return;

		const parsedPlaylists: Goober.Playlist[] = JSON.parse(storagePlaylists);

		selectedPlaylist.selected = false;
		chosenPlaylist.selected = true;

		// what the fuck? why do I need to assert?
		let selectedIndex = parsedPlaylists.findIndex((p) => p.name === selectedPlaylist!.name);
		let chosenIndex = parsedPlaylists.findIndex((p) => p.name === chosenPlaylist.name);

		parsedPlaylists[selectedIndex].selected = false;

		parsedPlaylists[chosenIndex].selected = true;

		localStorage.setItem('playlists', JSON.stringify(parsedPlaylists));
	}

	async function setShortcuts() {
		await register('CommandOrControl+Shift+ArrowLeft', async () => {
			await player.playPrevious();
		});

		await register('CommandOrControl+Shift+ArrowRight', async () => {
			await player.playNext();
		});
	}

	function selectSorting(
		e: Event & {
			currentTarget: EventTarget & HTMLSelectElement;
		}
	) {
		let value = (<HTMLInputElement>e.target).value;

		switch (value) {
			case 'Year': {
				sortBy(Sorting.ByYear);
				break;
			}
			case 'Artist (A-Z)': {
				sortBy(Sorting.ByArist);

				break;
			}
			case 'Album name (A-Z)': {
				sortBy(Sorting.ByAlbumName);

				break;
			}
		}
	}

	let elapsed = toTime($player.elapsed);
	let formattedElapsed = formatTime(elapsed);

	let duration = toTime($player.currentTrack?.duration || 0);
	let formattedDuration = formatTime(duration);

	let progress = '0';

	onMount(async () => {
		await appWindow.setTitle(`goober ${PKG.version}`);

		document.addEventListener('contextmenu', (event) => event.preventDefault());

		let storageLibrary = localStorage.getItem('library');
		let storagePlaylists = localStorage.getItem('playlists');

		if (!storageLibrary || !storagePlaylists) return;

		let parsedLibrary: Goober.Album[] = JSON.parse(storageLibrary);
		let parsedPlaylists: Goober.Playlist[] = JSON.parse(storagePlaylists);

		// if the playlists exist in localstorage there should in theory be a selected playlist
		// hence `!`
		currentPlaylist = parsedPlaylists.find((p) => p.selected == true)!;

		for (let album of currentPlaylist.content) {
			tracks.push(...album.tracks);
		}

		library = parsedLibrary;

		$player.library = library;
		$player.tracks = tracks;

		playlists = parsedPlaylists;

		// at the bottom due to it being less important
		let storageVolume = localStorage.getItem('volume');

		if (!storageVolume) {
			storageVolume = '1';

			localStorage.setItem('volume', storageVolume);
		}

		let volumeNumber = Number(storageVolume);

		$player.volume = volumeNumber;
		volume = volumeNumber * 100;

		await setShortcuts();

		await invoke('set_presence', {
			presence: {
				state: 'Browsing',
				details: `${library.reduce((acc, e) => acc + e.tracks.length, 0)} tracks loaded`,
				largeText: `v${PKG.version}`,
				smallImage: 'browsing'
			}
		});
	});

	$: {
		elapsed = toTime($player.elapsed);
		formattedElapsed = formatTime(elapsed);

		duration = toTime($player.currentTrack?.duration || 0);
		formattedDuration = formatTime(duration);

		if ($player.currentTrack) {
			let percentage = (100 * $player.elapsed) / $player.currentTrack.duration;
			if (percentage === Infinity) percentage = 0;
			progress = percentage ? percentage.toString() : '0';
		}
	}
</script>

<svelte:window on:keydown={onKeyDown} />

<div class="flex flex-col h-screen">
	<div class="flex flex-row h-[50px] flex-auto">
		<div
			class="flex flex-1 justify-center items-center border-r-2 border-gray-600"
			on:click={$player.playing ? player.pause : player.resume}
		>
			{#if $player.currentTrack?.trackAlbum.cover !== ''}
				<img
					src={convertFileSrc($player.currentTrack?.trackAlbum.cover || '')}
					class="h-full w-full"
				/>
			{/if}
		</div>

		<div class="flex-1 max-h-full overflow-auto gap-1">
			<div class="border-b-2 border-b-gray-500 mb-1 mt-2 sticky top-0 bg-zinc-900">
				<!---- <input
					type="text"
					placeholder="Search"
					class="input input-bordered w-full max-w-xs ml-1 mb-2"
				/>-->
				{#if currentPlaylist.name === '<all>'}
					<select
						class="select select-bordered max-w-xs w-1/3"
						on:change={selectSorting}
						bind:value={sortingMethod}
					>
						<option disabled selected>Sort by</option>
						<option>Year</option>
						<option>Artist (A-Z)</option>
						<option>Album name (A-Z)</option>
					</select>
				{/if}

				<select
					class="select select-bordered w-1/3 max-w-xs"
					on:change={selectPlaylist}
					bind:value={currentPlaylist.name}
				>
					<option disabled selected>Select playlist</option>
					{#each playlists as playlist}
						<option>{playlist.name}</option>
					{/each}
				</select>

				<CreatePlaylistModal bind:playlists />
				{#if currentPlaylist.name !== '<all>'}
					<!--playlistName is included in all of those so it's easier to just set currentPlayer and
					not worry about currentPlaylist.name changing. :)-->
					<AddTracksModal
						playlistName={currentPlaylist.name}
						bind:allPlaylist={playlists[0]}
						bind:tracks
						bind:currentPlaylist
						bind:player
					/>
					<EmptyPlaylistModal
						playlistName={currentPlaylist.name}
						bind:playlists
						bind:currentPlaylist
						bind:player
					/>
					<DeletePlaylistModal
						playlistName={currentPlaylist.name}
						bind:tracks
						bind:currentPlaylist
						bind:playlists
						bind:player
					/>
				{/if}
			</div>

			<div class="flex flex-col gap-y-1.5">
				{#each currentPlaylist.content as album}
					<Album {album} bind:player bind:allTracks={tracks} bind:currentPlaylist />
				{/each}
			</div>
		</div>
	</div>

	<div class="flex flex-col h-[95px] border-t-2 border-gray-600">
		<div class="flex flex-row mt-3 ml-5 gap-2 justify-between">
			<div class="flex flex-row gap-2">
				<h1 class="text-xl font-semibold text-gray-300 overflow-ellipsis whitespace-nowrap">
					{$player.currentTrack?.artist}
				</h1>
				<h1 class="text-xl font-light overflow-ellipsis whitespace-nowrap">
					{$player.currentTrack?.title}
				</h1>
			</div>

			<div class="flex flex-row mr-5">
				{#if $player.playing || $player.paused}
					<h1 class="text-xl whitespace-nowrap">
						<span class="font-semibold">{formattedElapsed}</span>
						<span class="font-thin">{formattedDuration}</span>
					</h1>
				{/if}
			</div>
		</div>

		<div class="h-2.5 px-5">
			<!-- svelte-ignore a11y-click-events-have-key-events -->
			<progress
				class="progress progress-accent mb-1.5"
				value={progress}
				max="100"
				on:click={progressBar}
			/>

			<Controls bind:player bind:volume />
		</div>
	</div>
</div>
