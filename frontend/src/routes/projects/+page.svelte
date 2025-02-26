<script lang="ts">
	import Card from '../../lib/components/card/Card.svelte';
	export let data;
</script>

<section class="mx-auto flex min-h-screen max-w-6xl flex-col justify-center px-4 py-10 sm:px-6">
	<div
		class="grid grid-cols-1 place-items-center gap-x-8 gap-y-6 px-2 sm:grid-cols-2 lg:grid-cols-3"
	>
		<Card></Card>
		<Card></Card>
		<Card></Card>
		<Card></Card>
		<Card></Card>
		<Card></Card>
		<Card></Card>
		<Card></Card>
	</div>
</section>
