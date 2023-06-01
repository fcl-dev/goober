<script lang="ts">
  import FaPlay from "svelte-icons/fa/FaPlay.svelte";
  import FaFastForward from "svelte-icons/fa/FaFastForward.svelte";
  import FaFastBackward from "svelte-icons/fa/FaFastBackward.svelte";
  import FaStop from "svelte-icons/fa/FaStop.svelte";
  import FaPause from "svelte-icons/fa/FaPause.svelte";

  import { invoke } from "@tauri-apps/api/tauri";
  import type { PlayerType } from "./player";

  export let player: PlayerType;

  const stop = () => {
    clearInterval($player.interval);
    console.log($player.interval);
    if ($player.element) $player.element.classList.remove("text-red-500");

    invoke("stop");

    player.default();
  };

  const pause = () => {
    $player.playing = false;
    $player.paused = true;

    invoke("pause");
  };

  const playBtn = () => {
    if ($player.paused) {
      invoke("resume");

      $player.playing = true;
      $player.paused = false;
      return;
    }

    invoke("play");
  };

  const fastForward = async () => {
    await player.playNext();
  };

  const fastBackward = async () => {
    await player.playPrevious();
  };
</script>

<div class="flex justify-center gap-0.5">
  <div
    class="h-6 w-6 border-2 border-gray-500 rounded-full bg-gray-800 text-gray-300 p-0.5 hover:cursor-pointer"
    on:click={fastBackward}
  >
    <FaFastBackward />
  </div>

  <div
    class="flex items-center justify-center h-6 w-6 border-2 border-gray-500 rounded-full bg-gray-800 text-gray-300 p-0.5 hover:cursor-pointer"
    on:click={stop}
  >
    <!-- svelte-ignore a11y-click-events-have-key-events -->
    <div class="h-3">
      <FaStop />
    </div>
  </div>

  <div
    class="flex items-center justify-center h-6 w-6 border-2 border-gray-500 rounded-full bg-gray-800 text-gray-300 p-0.5 hover:cursor-pointer"
    on:click={$player.playing ? pause : playBtn}
  >
    {#if $player.playing}
      <!-- svelte-ignore a11y-click-events-have-key-events -->
      <div class="h-3">
        <FaPause />
      </div>
    {:else}
      <div class="h-3">
        <FaPlay />
      </div>
    {/if}
  </div>

  <div
    class="h-6 w-6 border-2 border-gray-500 rounded-full bg-gray-800 text-gray-300 p-0.5 hover:cursor-pointer"
    on:click={fastForward}
  >
    <FaFastForward />
  </div>
</div>
