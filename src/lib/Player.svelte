<script lang="ts">
  import Album from "./Album.svelte";
  import Controls from "./Controls.svelte";
  import { listen } from "@tauri-apps/api/event";
  import { convertFileSrc, invoke } from "@tauri-apps/api/tauri";
  import { Player } from "./player";
  import { onMount } from "svelte";

  let player = Player();
  let allTracks: Track[] = [];

  function progressBar(event: MouseEvent) {
    const progressBar = event.target as HTMLElement;
    const rect = progressBar.getBoundingClientRect();
    const x = event.clientX - rect.left;
    const percentage = (x / rect.width) * 100;
    console.log(`Clicked at ${percentage}%`);
  }

  const onKeyDown = (e: KeyboardEvent) => {
    if (e.code === "Space" && $player.playing) {
      $player.playing = false;
      $player.paused = true;

      invoke("pause");
    } else if (e.code === "Space" && $player.paused) {
      $player.playing = true;
      $player.paused = false;

      invoke("resume");
    }
  };

  let payload: Payload = {
    albums: [],
  };

  listen("music", (event) => {
    payload = event.payload as Payload;
    allTracks = [];

    for (let album of payload.albums) {
      allTracks.push(...album.tracks);
    }

    localStorage.clear();
    localStorage.setItem("albums", JSON.stringify(payload.albums));
  });

  type Time = {
    h: number;
    m: number;
    s: number;
  };

  function toTime(totalSeconds: number): Time {
    const totalMinutes = Math.floor(totalSeconds / 60);

    const seconds = totalSeconds % 60;
    const hours = Math.floor(totalMinutes / 60);
    const minutes = totalMinutes % 60;

    return { h: hours, m: minutes, s: seconds };
  }

  function formatTime(time: Time): string {
    let h = time.h.toString();
    let m = time.m.toString();
    let s = time.s.toString();

    if (h === "0") return `${m.padStart(2, "0")}:${s.padStart(2, "0")}`;

    return `${h.padStart(2, "0")}:${m.padStart(2, "0")}:${s.padStart(2, "0")}`;
  }

  let elapsed = toTime($player.elapsed);
  let formattedElapsed = formatTime(elapsed);

  let duration = toTime($player.currentTrack.duration);
  let formattedDuration = formatTime(duration);

  let progress = "0";

  onMount(() => {
    let albums = localStorage.getItem("albums");

    if (!albums) return;

    payload.albums = JSON.parse(albums);

    for (let album of payload.albums) {
      allTracks.push(...album.tracks);
    }
  });

  $: {
    elapsed = toTime($player.elapsed);
    formattedElapsed = formatTime(elapsed);

    duration = toTime($player.currentTrack.duration);
    formattedDuration = formatTime(duration);

    let percentage = (100 * $player.elapsed) / $player.currentTrack.duration;
    if (percentage === Infinity) percentage = 0;
    progress = percentage ? percentage.toString() : "0";
  }
</script>

<svelte:window on:keydown={onKeyDown} />

<div class="flex flex-col h-screen">
  <div class="flex flex-row h-[50px] flex-auto">
    <div
      class="flex flex-1 justify-center items-center border-r-2 border-gray-600"
    >
      {#if $player.currentTrack.album.cover !== ""}
        <img
          src={convertFileSrc($player.currentTrack.album.cover)}
          class="h-full w-full"
        />
      {/if}
    </div>

    <div class="flex-1 max-h-full overflow-auto gap-1">
      {#each payload.albums as album}
        <Album {album} bind:player bind:allTracks />
      {/each}
    </div>
  </div>

  <div class="flex flex-col h-[95px] border-t-2 border-gray-600">
    <div class="flex flex-row mt-3 ml-5 gap-2 justify-between">
      <div class="flex flex-row gap-2">
        <h1 class="text-xl font-semibold text-gray-300">
          {$player.currentTrack.artist}
        </h1>
        <h1 class="text-xl font-light">{$player.currentTrack.title}</h1>
      </div>

      <div class="flex flex-row mr-5">
        {#if $player.playing || $player.paused}
          <h1 class="text-xl">
            <span class="font-semibold">{formattedElapsed}</span>
            <span class="font-thin">{formattedDuration}</span>
          </h1>
        {/if}
      </div>
    </div>

    <div class="h-2.5 px-5">
      <!-- svelte-ignore a11y-click-events-have-key-events -->
      <progress
        class="progress progress-primary mb-1.5"
        value={progress}
        max="100"
        on:click={progressBar}
      />

      <Controls bind:player />
    </div>
  </div>
</div>
