<script lang="ts">
	export let optionName: string;
	export let displayName: string;

	let checked = false;

	$: localStorage.setItem(`preferences.${optionName}`, String(checked));
</script>

<div class="flex flex-row gap-1 justify-center items-center">
	<span>{displayName}</span>
	<input type="checkbox" bind:checked class="checkbox checkbox-success" />
</div>
