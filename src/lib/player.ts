import { invoke } from "@tauri-apps/api/tauri";

export type PlayerType = {
  play(track: Track): Promise<void>;
  playPrevious(): Promise<void>;
  playNext(): Promise<void>;
  default(): Promise<void>;
  subscribe(subscriber: Subscriber): () => void;
  set(newPlayer: PlayerObj): void;
  update(x: (player: PlayerObj) => void): void;
};

export type PlayerObj = {
  track?: Track;
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
    track: {
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

  let returns = {
    async play(track: Track) {
      await invoke("play", {
        path: track.path,
      });

      console.log(track);

      clearInterval(player.interval);
      player.elapsed = 0;

      player.interval = setInterval(async () => {
        if (!player.playing) return;

        if (
          player.elapsed === player.track.duration &&
          player.track.duration !== 0
        ) {
          clearInterval(player.interval);

          await returns.playNext();
        }

        player.elapsed++;

        subscribers.forEach((subscriber) => {
          subscriber(player);
          console.log(player.elapsed);
        });
      }, 1000);

      player.track = track;
      player.playing = true;
      player.paused = false;

      if (player.element) player.element.classList.add("text-red-500");

      subscribers.forEach((subscriber) => {
        subscriber(player);
        console.log(subscribers);
      });
    },

    async playPrevious() {
      player.i--;
      player.track = player.tracks[player.i];

      if (!player.track) {
        player.i = player.tracks.length - 1;
        player.track = player.tracks[player.i];
      }

      returns.play(player.track);

      subscribers.forEach((subscriber) => {
        subscriber(player);
      });
    },

    async playNext() {
      player.i++;
      player.track = player.tracks[player.i];

      if (!player.track) {
        player.i = 0;
        player.track = player.tracks[player.i];
      }

      returns.play(player.track);

      subscribers.forEach((subscriber) => {
        subscriber(player);
      });
    },
    async default() {
      player = {
        track: {
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

      subscribers.forEach((subscriber) => {
        subscriber(player);
      });
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
      subscribers.forEach((subscriber) => {
        subscriber(player);
      });
    },
    update(x: (player: PlayerObj) => void) {
      x(player);

      subscribers.forEach((subscriber) => {
        subscriber(player);
      });
    },
  };

  return returns;
}
