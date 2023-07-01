// See https://kit.svelte.dev/docs/types#app
// for information about these interfaces
declare global {
	namespace App {
		// interface Error {}
		// interface Locals {}
		// interface PageData {}
		// interface Platform {}
	}
	namespace Goober {
		/// Different to Album. TrackAlbum is short, only for playback.
		type TrackAlbum = {
			cover: string;
		};

		type Playlist = {
			name: string;
			selected: boolean;
			content: Goober.Album[];
			cover: string;
		};

		type Track = {
			track: string;
			trackAlbum: TrackAlbum;
			artist: string;
			duration: number;
			path: string;
			title: string;
		};

		type Album = {
			name: string;
			artist: string;
			cover: string;
			year: number;
			tracks: Array<Track>;
		};

		type Payload = {
			library: Album[];
		};
	}
}

export {};
