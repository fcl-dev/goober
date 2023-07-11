import { invoke } from '@tauri-apps/api/tauri';
import { setInterval, clearInterval } from 'worker-timers';

export type Player = {
	currentTrack?: Goober.Track;
	tracks: Goober.Track[];
	library: Goober.Album[];
	i: number;
	playing: boolean;
	paused: boolean;
	elapsed: number;
	shuffling: boolean;
	volume: number;
	interval?: ReturnType<typeof setInterval>;
	element?: HTMLElement;
};

type Subscriber = (player: Player) => void;

export function Player() {
	const subscribers: Set<Subscriber> = new Set();

	let player: Player = {
		currentTrack: {
			track: '',
			trackAlbum: {
				cover: ''
			},

			artist: 'goober',
			duration: 0,
			path: '',
			title: `version ${PKG.version}`
		},
		shuffling: false,
		tracks: [],
		library: [],
		i: -1,
		volume: 1,
		playing: false,
		paused: false,
		elapsed: 0
	};

	const announce = () => {
		subscribers.forEach((subscriber) => {
			subscriber(player);
		});
	};

	const methods = {
		async tryClearInterval() {
			if (!player.interval) return;

			clearInterval(player.interval);

			player.interval = undefined;

			announce();
		},
		async play(track: Goober.Track) {
			await invoke('play', {
				path: track.path,
				volume: player.volume
			});

			methods.tryClearInterval();
			player.elapsed = 0;

			player.interval = setInterval(async () => {
				if (!player.playing) return;

				if (
					player.elapsed === player.currentTrack?.duration &&
					player.currentTrack?.duration !== 0
				) {
					methods.tryClearInterval();

					await methods.playNext();
				}

				player.elapsed++;

				announce();
			}, 1000);

			player.currentTrack = track;
			player.playing = true;
			player.paused = false;

			if (player.element) {
				player.element.classList.add('text-blue-400');
			}

			announce();

			await invoke('set_presence', {
				presence: {
					state: 'Playing',
					details: `${player.currentTrack.artist} - ${player.currentTrack.title}`,
					largeText: `v${PKG.version}`
				}
			});
		},
		async resume() {
			if (!player.paused) return;

			player.playing = true;
			player.paused = false;

			announce();

			await invoke('resume');
			await invoke('set_presence', {
				presence: {
					state: 'Playing',
					details: `${player.currentTrack?.artist} - ${player.currentTrack?.title}`,
					largeText: `v${PKG.version}`
				}
			});
		},
		async pause() {
			player.playing = false;
			player.paused = true;

			announce();

			await invoke('pause');
			await invoke('set_presence', {
				presence: {
					state: 'Paused',
					details: `${player.currentTrack?.artist} - ${player.currentTrack?.title}`,
					largeText: `v${PKG.version}`
				}
			});
		},
		async playPrevious() {
			player.i--;
			player.currentTrack = player.tracks[player.i];

			player.element?.classList.remove('text-blue-400');

			const elements = [
				...(<HTMLCollectionOf<HTMLElement>>document.getElementsByClassName('track'))
			];

			const previousElement = elements[player.i];

			player.element = previousElement;

			if (!player.currentTrack) {
				player.i = player.tracks.length - 1;
				player.currentTrack = player.tracks[player.i];

				player.element = elements[player.i];
			}

			methods.play(player.currentTrack);

			announce();
			return;
		},

		async playNext() {
			if (!player.shuffling) {
				player.i++;
				player.currentTrack = player.tracks[player.i];

				player.element?.classList.remove('text-blue-400');

				const elements = [
					...(<HTMLCollectionOf<HTMLElement>>document.getElementsByClassName('track'))
				];

				const nextElement = elements[player.i];

				player.element = nextElement;

				if (!player.currentTrack) {
					player.i = 0;
					player.currentTrack = player.tracks[player.i];

					player.element = elements[player.i];
				}

				methods.play(player.currentTrack);

				announce();

				return;
			}

			player.i = Math.floor(Math.random() * player.tracks.length);
			player.currentTrack = player.tracks[player.i];

			player.element?.classList.remove('text-blue-400');

			const elements = [
				...(<HTMLCollectionOf<HTMLElement>>document.getElementsByClassName('track'))
			];

			const nextElement = elements[player.i];
			player.element = nextElement;

			methods.play(player.currentTrack);
			announce();
		},
		async stop() {
			methods.tryClearInterval();

			if (player.element) player.element.classList.remove('text-blue-400');

			await invoke('stop');

			await invoke('set_presence', {
				presence: {
					state: 'Browsing',
					details: `${player.library.reduce((acc, e) => acc + e.tracks.length, 0)} tracks loaded`,
					largeText: `v${PKG.version}`
				}
			});

			methods.default();

			announce();
		},
		async updateElement(element: HTMLElement) {
			if (player.element) player.element.classList.remove('text-blue-400');

			player.element = element;
			player.element.classList.add('text-blue-400');

			announce();
		},
		async default() {
			player = {
				currentTrack: {
					track: '',
					trackAlbum: {
						cover: ''
					},

					artist: 'goober',
					duration: 0,
					path: '',
					title: `version ${PKG.version}`
				},
				shuffling: false,
				tracks: [],
				library: [],
				i: -1,
				volume: player.volume,
				playing: false,
				paused: false,
				elapsed: 0
			};

			announce();
		},
		subscribe(subscriber: Subscriber) {
			subscribers.add(subscriber);
			subscriber(player);

			return () => {
				subscribers.delete(subscriber);
			};
		},
		set(newPlayer: Player) {
			player = newPlayer;

			announce();
		},
		update(x: (player: Player) => void) {
			x(player);

			announce();
		}
	};

	return methods;
}
