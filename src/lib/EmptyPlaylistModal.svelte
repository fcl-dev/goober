<script lang="ts">
	import FaEraser from 'svelte-icons/fa/FaEraser.svelte';
	import type { Player } from './player';

	export let playlistName: string;
	export let playlists: Goober.Playlist[];
	export let currentPlaylist: Goober.Playlist;
	export let player: ReturnType<typeof Player>;

	function openEmptyModal() {
		const emptyModal = <HTMLFormElement>document.querySelector('#empty_modal');

		emptyModal.showModal();
	}

	function emptyPlaylist() {
		let currentIndex = playlists.findIndex((p) => p.name === playlistName);

		currentPlaylist.content[0].tracks = [];
		playlists[currentIndex].content[0].tracks = [];

		$player.tracks = [];
		$player.i = -1;

		localStorage.setItem('playlists', JSON.stringify(playlists));
	}
</script>

<button class="btn" on:click={openEmptyModal}>
	<div class="h-3 w-3">
		<FaEraser />
	</div>
</button>

<dialog id="empty_modal" class="modal">
	<form method="dialog" class="modal-box">
		<h3 class="text-lg">
			Are you sure you want to empty the <strong>{currentPlaylist.name}</strong> playlist's contents?
		</h3>
		<p class="py-4">
			This action <strong>cannot</strong> be undone. Ever. All the content in the playlist will vanish
			forever just like that. Poof.
		</p>
		<div class="modal-action">
			<button class="btn btn-info">Cancel</button>
			<button class="btn btn-error" on:click={emptyPlaylist}>Empty</button>
		</div>
	</form>
</dialog>
