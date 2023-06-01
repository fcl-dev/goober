/// <reference types="svelte" />
/// <reference types="vite/client" />

declare module "@fortawesome/pro-solid-svg-icons/index.es" {
  export * from "@fortawesome/pro-solid-svg-icons";
}

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
  tracks: Array<Track>;
};

type Payload = {
  albums: Album[];
};
