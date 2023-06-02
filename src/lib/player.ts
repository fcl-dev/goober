import { invoke } from "@tauri-apps/api/tauri";

export type PlayerType = {
  play(track: Track): Promise<void>;
  playPrevious(): Promise<void>;
  playNext(): Promise<void>;
  updateElement(element: HTMLElement): Promise<void>;
  default(): Promise<void>;
  subscribe(subscriber: Subscriber): () => void;
  set(newPlayer: PlayerObj): void;
  update(x: (player: PlayerObj) => void): void;
};

export type PlayerObj = {
  currentTrack?: Track;
  tracks: Track[];
  i: number;
  playing: boolean;
  paused: boolean;
  elapsed: number;
  interval?: ReturnType<typeof setInterval>;
  element?: HTMLElement;
};

type Subscriber = (player: PlayerObj) => void;

export function Player(): PlayerType {
  const subscribers: Set<Subscriber> = new Set();

  let player: PlayerObj = {
    currentTrack: {
      track: "",
      album: {
        name: "goober",
        artist: "goober",
        cover: "",
        tracks: [],
      },

      artist: "goober",
      duration: 0,
      path: "",
      title: "version pre-a1.0",
    },
    tracks: [],
    i: 0,
    playing: false,
    paused: false,
    elapsed: 0,
  };

  let announce = () => {
    subscribers.forEach((subscriber) => {
      subscriber(player);
    });
  };

  let methods = {
    async play(track: Track) {
      await invoke("play", {
        path: track.path,
      });

      clearInterval(player.interval);
      player.elapsed = 0;

      player.interval = setInterval(async () => {
        if (!player.playing) return;

        if (
          player.elapsed === player.currentTrack.duration &&
          player.currentTrack.duration !== 0
        ) {
          clearInterval(player.interval);

          await methods.playNext();
        }

        player.elapsed++;

        announce();
      }, 1000);

      player.currentTrack = track;
      player.playing = true;
      player.paused = false;

      if (player.element) player.element.classList.add("text-cyan-400");

      announce();
    },

    async playPrevious() {
      player.i--;
      player.currentTrack = player.tracks[player.i];

      player.element.classList.remove("text-cyan-400");

      let elements = [
        ...(document.getElementsByClassName(
          "track"
        ) as HTMLCollectionOf<HTMLElement>),
      ];

      let index = elements.findIndex((x) => x === player.element) - 1;
      let previousElement = elements[index];

      player.element = previousElement;

      if (!player.currentTrack) {
        player.i = player.tracks.length - 1;
        player.currentTrack = player.tracks[player.i];

        player.element = elements[player.i];
      }

      methods.play(player.currentTrack);

      announce();
    },

    async playNext() {
      player.i++;
      player.currentTrack = player.tracks[player.i];

      player.element.classList.remove("text-cyan-400");

      let elements = [
        ...(document.getElementsByClassName(
          "track"
        ) as HTMLCollectionOf<HTMLElement>),
      ];

      let index = elements.findIndex((x) => x === player.element) + 1;
      let nextElement = elements[index];

      player.element = nextElement;

      if (!player.currentTrack) {
        player.i = 0;
        player.currentTrack = player.tracks[player.i];

        player.element = elements[player.i];
      }

      methods.play(player.currentTrack);

      announce();
    },
    async updateElement(element: HTMLElement) {
      if (player.element) player.element.classList.remove("text-cyan-400");

      player.element = element;
      player.element.classList.add("text-cyan-400");

      announce();
    },
    async default() {
      player = {
        currentTrack: {
          track: "",
          album: {
            name: "goober",
            artist: "goober",
            cover: "",
            tracks: [],
          },

          artist: "goober",
          duration: 0,
          path: "",
          title: "version pre-a1.0",
        },
        tracks: [],
        i: 0,
        playing: false,
        paused: false,
        elapsed: 0,
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
    set(newPlayer: PlayerObj) {
      player = newPlayer;

      announce();
    },
    update(x: (player: PlayerObj) => void) {
      x(player);

      announce();
    },
  };

  return methods;
}
