<script lang="ts">
  import Album from "./Album.svelte";
  import Controls from "./Controls.svelte";
  import { listen } from "@tauri-apps/api/event";
  import { convertFileSrc, invoke } from "@tauri-apps/api/tauri";
  import { Player } from "./player";

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

    console.log(allTracks);
  });

  function formatTime(timeString: string): string {
    const length = timeString.length;

    if (length === 1) {
      return `0:0${timeString}`;
    } else if (length === 2) {
      return `0:${timeString}`;
    } else if (length === 3) {
      return `${timeString[0]}:${timeString.substring(1)}`;
    } else if (length === 4) {
      return `${timeString[0] + timeString[1]}:${timeString.substring(2)}`;
    } else if (length > 4) {
      const hours = timeString.substring(0, length - 4);
      const minutes = timeString.substring(length - 4, length - 2);
      const seconds = timeString.substring(length - 2);
      return `${hours}:${minutes}:${seconds}`;
    }
  }

  let formattedElapsed = formatTime($player.elapsed.toString());
  let formattedDuration = formatTime($player.track.duration.toString());
  let progress = "0";

  $: {
    formattedElapsed = formatTime($player.elapsed.toString());
    formattedDuration = formatTime($player.track.duration.toString());

    let percentage = (100 * $player.elapsed) / $player.track.duration;
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
      {#if $player.track.album.cover !== ""}
        <img
          src={convertFileSrc($player.track.album.cover)}
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
          {$player.track.artist}
        </h1>
        <h1 class="text-xl font-light">{$player.track.title}</h1>
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
