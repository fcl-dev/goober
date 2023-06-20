<script lang="ts">
	import Album from './Album.svelte';
	import Controls from './Controls.svelte';
	import { listen } from '@tauri-apps/api/event';
	import { convertFileSrc, invoke } from '@tauri-apps/api/tauri';
	import { Player } from './player';
	import { onMount } from 'svelte';

	let player = Player();
	let allTracks: Goober.Track[] = [];

	function progressBar(event: MouseEvent) {
		const progressBar = event.target as HTMLElement;
		const rect = progressBar.getBoundingClientRect();
		const x = event.clientX - rect.left;
		const percentage = (x / rect.width) * 100;
	}

	const onKeyDown = (e: KeyboardEvent) => {
		if (e.code === 'Space' && $player.playing) {
			player.pause();
		} else if (e.code === 'Space' && $player.paused) {
			player.resume();
		}
	};

	enum Sorting {
		ByYear,
		ByArist,
		ByAlbumName
	}

	function sortBy(by: Sorting) {
		switch (by) {
			case Sorting.ByYear: {
				albums.sort((a, b) => (a.year > b.year ? 1 : -1));
				albums = albums;

				allTracks = albums.flatMap((album) => album.tracks);
				break;
			}
			case Sorting.ByArist: {
				albums.sort((a, b) => (a.artist > b.artist ? 1 : -1));
				albums = albums;

				allTracks = albums.flatMap((album) => album.tracks);
				break;
			}
			case Sorting.ByAlbumName: {
				albums.sort((a, b) => (a.name > b.name ? 1 : -1));
				albums = albums;

				allTracks = albums.flatMap((album) => album.tracks);
				break;
			}
		}
	}

	let albums: Goober.Album[] = [];

	listen('music', (event) => {
		albums = (event.payload as Goober.Payload).albums;

		sortBy(Sorting.ByYear);

		$player.tracks = allTracks;

		localStorage.clear();
		localStorage.setItem('albums', JSON.stringify(albums));
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

	let elapsed = toTime($player.elapsed);
	let formattedElapsed = formatTime(elapsed);

	let duration = toTime($player.currentTrack?.duration || 0);
	let formattedDuration = formatTime(duration);

	let progress = '0';

	onMount(() => {
		document.addEventListener('contextmenu', (event) => event.preventDefault());

		let storageAlbums = localStorage.getItem('albums');

		if (!storageAlbums) return;

		let parsedAlbums = JSON.parse(storageAlbums);

		for (let album of parsedAlbums) {
			allTracks.push(...album.tracks);
		}

		$player.tracks = allTracks;
		console.log(allTracks.length, $player.tracks.length);
		albums = parsedAlbums;
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
			{#if $player.currentTrack?.album.cover !== ''}
				<img src={convertFileSrc($player.currentTrack?.album.cover || '')} class="h-full w-full" />
			{/if}
		</div>

		<div class="flex-1 max-h-full overflow-auto gap-1">
			<div class="border-b-2 border-b-gray-500 mb-1 mt-2">
				<input
					type="text"
					placeholder="Search"
					class="input input-bordered w-full max-w-xs ml-1 mb-2"
				/>
				<div class="dropdown">
					<label tabindex="0" class="btn m-1">Sort by:</label>
					<ul tabindex="0" class="dropdown-content menu p-2 shadow bg-base-100 rounded-box w-52">
						<li on:click={() => sortBy(Sorting.ByYear)}><a>Year</a></li>
						<li>
							<a on:click={() => sortBy(Sorting.ByArist)}>Artist (A-Z)</a>
						</li>
						<li on:click={() => sortBy(Sorting.ByAlbumName)}>
							<a>Album Name</a>
						</li>
					</ul>
				</div>
			</div>

			<div>
				{#each albums as album}
					<Album {album} bind:player bind:allTracks />
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

			<Controls bind:player />
		</div>
	</div>
</div>
