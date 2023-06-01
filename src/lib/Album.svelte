<script lang="ts">
  import { convertFileSrc } from "@tauri-apps/api/tauri";
  import type { PlayerType } from "./player";

  export let player: PlayerType;
  export let album: Album;
  export let allTracks: Track[];

  let name = album.name;
  let artist = album.artist;
  let cover = album.cover;
  let tracks = album.tracks;

  $: name = album.name;
  $: artist = album.artist;
  $: cover = album.cover;
  $: tracks = album.tracks;

  function toMs(number: number) {
    const paddedNumber = number.toString().padStart(7, "0");
    const hours = parseInt(paddedNumber.slice(0, -4));
    const minutes = parseInt(paddedNumber.slice(-4, -2));
    const seconds = parseInt(paddedNumber.slice(-2));

    // convert the shitty format to millis
    const ms = hours * 3.6e6 + minutes * 60000 + seconds * 1000;

    return ms;
  }

  async function playTrack(track: Track, event: MouseEvent) {
    console.log("rerun");
    const target = event.target as HTMLElement;

    clearInterval($player.interval);

    if ($player.element) $player.element.classList.remove("text-red-500");

    target.classList.add("text-red-500");

    let i = allTracks.findIndex((t) => t == track);

    $player.i = i;
    $player.tracks = allTracks;
    $player.element = target;

    player.play(track);
  }
</script>

<div class="flex flex-1">
  {#if cover !== ""}
    <img src={convertFileSrc(cover)} class="h-28 w-30" />
  {/if}
  <div class="flex flex-col ml-2">
    <h1 class="mt-2 text-3xl">{name}</h1>
    <h2>{artist}</h2>

    <div class="mt-2">
      {#each tracks as track}
        <!-- svelte-ignore a11y-click-events-have-key-events -->
        <h1
          class="hover:cursor-pointer"
          on:click={(event) => playTrack(track, event)}
          id="track"
        >
          {track.track}
          {track.title}
        </h1>
      {/each}
    </div>
  </div>
</div>
