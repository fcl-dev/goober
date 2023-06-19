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
		type Track = {
			track: string;
			album: Album;
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
			albums: Album[];
		};
	}
}

export {};
