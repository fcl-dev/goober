import { invoke } from "@tauri-apps/api/tauri";
import { setInterval, clearInterval } from "worker-timers";

export type Player = {
  currentTrack?: Goober.Track;
  tracks: Goober.Track[];
  i: number;
  playing: boolean;
  paused: boolean;
  elapsed: number;
  shuffling: boolean;
  interval?: ReturnType<typeof setInterval>;
  element?: HTMLElement;
};

type Subscriber = (player: Player) => void;

export function Player() {
  const subscribers: Set<Subscriber> = new Set();

  let player: Player = {
    currentTrack: {
      track: "",
      album: {
        name: "goober",
        artist: "goober",
        cover: "",
        year: 0,
        tracks: [],
      },

      artist: "goober",
      duration: 0,
      path: "",
      title: "version pre-a1.0",
    },
    shuffling: false,
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
    async tryClearInterval() {
      if (!player.interval) return;

      clearInterval(player.interval);

      player.interval = null;

      announce();
    },
    async play(track: Goober.Track) {
      await invoke("play", {
        path: track.path,
      });

      methods.tryClearInterval();
      player.elapsed = 0;

      player.interval = setInterval(async () => {
        if (!player.playing) return;

        if (player.elapsed === player.currentTrack.duration) {
          methods.tryClearInterval();

          await methods.playNext();
        }

        player.elapsed++;

        announce();
      }, 1000);

      player.currentTrack = track;
      player.playing = true;
      player.paused = false;

      if (player.element) player.element.classList.add("text-blue-400");

      announce();
    },

    async playPrevious() {
      player.i--;
      player.currentTrack = player.tracks[player.i];

      player.element.classList.remove("text-blue-400");

      let elements = [
        ...(document.getElementsByClassName(
          "track"
        ) as HTMLCollectionOf<HTMLElement>),
      ];

      let previousElement = elements[player.i];

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

        player.element.classList.remove("text-blue-400");

        let elements = [
          ...(document.getElementsByClassName(
            "track"
          ) as HTMLCollectionOf<HTMLElement>),
        ];

        let nextElement = elements[player.i];

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

      player.element.classList.remove("text-blue-400");

      let elements = [
        ...(document.getElementsByClassName(
          "track"
        ) as HTMLCollectionOf<HTMLElement>),
      ];

      let nextElement = elements[player.i];
      player.element = nextElement;

      methods.play(player.currentTrack);
      announce();
    },
    async updateElement(element: HTMLElement) {
      if (player.element) player.element.classList.remove("text-blue-400");

      player.element = element;
      player.element.classList.add("text-blue-400");

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
            year: 0,
            tracks: [],
          },

          artist: "goober",
          duration: 0,
          path: "",
          title: "version pre-a1.0",
        },
        shuffling: false,
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
    set(newPlayer: Player) {
      player = newPlayer;

      announce();
    },
    update(x: (player: Player) => void) {
      x(player);

      announce();
    },
  };

  return methods;
}
