<script lang="ts">
	import { open } from '@tauri-apps/api/shell';
</script>

<div class="h-screen">
	<div class="flex flex-row gap-1">
		<img src="/goober-icon-transparent-white.png" class="w-32 h-32" />
		<div class="flex items-center">
			<h1 class="text-3xl">About <span class="font-bold">goober</span></h1>
		</div>
	</div>
	<div class="mt-2">
		<!-- svelte-ignore missing-declaration -->
		<h1 class="text-2xl">
			Running goober v{PKG.version} by FCL.<span class="text-[#00ff88]">dev</span>
		</h1>
		<a
			on:click={() => open('https://github.com/fcl-dev/goober')}
			class="text-blue-400 hover:text-blue-200 transition-all text-2xl hover:cursor-pointer"
		>
			GitHub
		</a>
	</div>

	<div class="flex justify-center mt-7">
		<img src="/FCLogo.png" class="w-52 h-52" />
	</div>
</div>
