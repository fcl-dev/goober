<script lang="ts">
	import { open } from '@tauri-apps/api/dialog';

	let name = '';
	let cover = '';

	export let playlists: Goober.Playlist[];

	function createPlaylist() {
		if (name === '<all>') return;

		const storagePlaylists = localStorage.getItem('playlists');

		if (!storagePlaylists) return;

		const parsedPlaylists: Goober.Playlist[] = JSON.parse(storagePlaylists);

		if (parsedPlaylists.find((p) => p.name === name)) {
			return;
		}

		let playlist: Goober.Playlist = {
			name,
			selected: false,
			content: [
				{
					name,
					artist: '',
					cover,
					year: 0,
					tracks: []
				}
			],
			cover
		};

		playlists.push(playlist);

		parsedPlaylists.push(playlist);

		localStorage.setItem('playlists', JSON.stringify(parsedPlaylists));

		playlists = playlists;

		let modal = <HTMLFormElement>document.querySelector('#create_modal');

		modal.close();
	}

	function openCreationModal() {
		let modal = <HTMLFormElement>document.querySelector('#create_modal');

		modal.showModal();
	}

	async function selectCover() {
		let path = <string | null>await open({
			multiple: false,
			filters: [
				{
					name: 'All Files',
					extensions: ['*']
				}
			]
		});

		cover = path ?? '';
	}
</script>

<button class="btn" on:click={openCreationModal}>Create playlist</button>
<dialog id="create_modal" class="modal">
	<form method="dialog" class="modal-box">
		<h3 class="font-bold text-lg">Create playlist</h3>
		<div class="flex flex-col items-center">
			<div class="form-control w-full max-w-xs">
				<label class="label">
					<span class="label-text">Playlist name</span>
				</label>
				<input
					type="text"
					placeholder="Name"
					class="input input-bordered input-primary w-full max-w-xs mt-2"
					bind:value={name}
				/>
				<label class="label">
					<span class="label-text">Cover image</span>
				</label>

				<div class="flex items-center gap-1">
					<input
						type="button"
						class="btn btn-primary w-5/6"
						value="Select"
						on:click={selectCover}
					/>
					<input
						type="button"
						class="btn btn-warning mt-1 w-1/6"
						value="Clear"
						on:click={() => (cover = '')}
					/>
				</div>
			</div>
			<h1>{cover !== '' ? `${cover.replace(/^.*[\\\/]/, '')} selected` : 'No image selected'}</h1>

			<button class="btn btn-primary mt-2" on:click={createPlaylist}>Create</button>
		</div>
	</form>
	<form method="dialog" class="modal-backdrop">
		<button>close</button>
	</form>
</dialog>
