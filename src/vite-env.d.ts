/// <reference types="svelte" />
/// <reference types="vite/client" />

declare module "@fortawesome/pro-solid-svg-icons/index.es" {
  export * from "@fortawesome/pro-solid-svg-icons";
}

declare namespace Goober {
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
