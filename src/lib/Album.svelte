<script lang="ts">
  import { convertFileSrc } from "@tauri-apps/api/tauri";
  import type { PlayerType } from "./player";

  export let player: PlayerType;
  export let album: Album;
  export let allTracks: Track[];

  let name = album.name;
  let artist = album.artist;
  let cover = album.cover;
  let year = album.year;
  let tracks = album.tracks;

  $: name = album.name;
  $: artist = album.artist;
  $: cover = album.cover;
  $: year = album.year;
  $: tracks = album.tracks;

  async function playTrack(track: Track, event: MouseEvent) {
    const targetHTML = event.target as HTMLElement;
    const target =
      targetHTML.children.length > 0 ? targetHTML : targetHTML.parentElement;

    player.tryClearInterval();

    player.updateElement(target);

    let i = allTracks.findIndex((t) => t == track);

    $player.i = i;
    $player.tracks = allTracks;
    $player.element = target;

    player.play(track);
  }
</script>

<div class="flex flex-1">
  {#if cover !== ""}
    <img src={convertFileSrc(cover)} class="h-28 w-30 transform-gpu" />
  {/if}
  <div class="flex flex-col ml-2 w-full">
    <div class="flex justify-between">
      <h1 class="mt-2 text-3xl">{name}</h1>
      {#if year != 0}
        <h1 class="mt-2 text-3xl mr-5">{year}</h1>
      {/if}
    </div>

    <h2>{artist}</h2>

    <div class="mt-2">
      {#each tracks as track}
        <!-- svelte-ignore a11y-click-events-have-key-events -->
        <div
          class="flex hover:cursor-pointer hover:text-gray-400 transition-all track gap-1"
          on:click={(event) => playTrack(track, event)}
        >
          {#if track.track !== ""}
            <span>
              {track.track}.
            </span>
          {/if}

          <span>
            {track.title}
          </span>
        </div>
      {/each}
    </div>
  </div>
</div>
