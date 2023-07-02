<script lang="ts">
	import FaTrash from 'svelte-icons/fa/FaTrash.svelte';
	import type { Player } from './player';

	export let playlistName: string;
	export let tracks: Goober.Track[];
	export let currentPlaylist: Goober.Playlist;
	export let playlists: Goober.Playlist[];
	export let player: ReturnType<typeof Player>;

	function openDeleteModal() {
		const deleteModal = <HTMLFormElement>document.querySelector('#delete_modal');

		deleteModal.showModal();
	}

	function deletePlaylist() {
		let allPlaylist = playlists[0];

		currentPlaylist = allPlaylist;

		currentPlaylist.selected = true;

		playlists = playlists.filter((p) => p.name !== playlistName);

		playlists[playlists.indexOf(allPlaylist)].selected = true;

		tracks = currentPlaylist.content.flatMap((c) => c.tracks);

		$player.i = -1;
		$player.tracks = tracks;

		currentPlaylist = currentPlaylist;

		localStorage.setItem('playlists', JSON.stringify(playlists));
	}
</script>

<button class="btn" on:click={openDeleteModal}><div class="w-3 h-3"><FaTrash /></div></button>
<dialog id="delete_modal" class="modal">
	<form method="dialog" class="modal-box">
		<h3 class="text-lg">
			Are you sure you want to delete the <strong>{playlistName}</strong> playlist?
		</h3>
		<p class="py-4">This action <strong>cannot</strong> be undone. Ever. It just goes poof.</p>
		<div class="modal-action">
			<button class="btn btn-info">Cancel</button>
			<button class="btn btn-error" on:click={deletePlaylist}>Delete</button>
		</div>
	</form>
</dialog>
