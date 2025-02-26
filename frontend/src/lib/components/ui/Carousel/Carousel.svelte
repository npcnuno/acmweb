<!-- Carousel.svelte -->
<script lang="ts">
	import { onMount } from 'svelte';
	import Markdown from 'svelte-markdown';

	export let slides: Array<{
		id: number;
		description: string;
		descriptionPosition: string;
		image: { alt: string; src: string };
	}>;

	let index = 0;
	let container: HTMLDivElement;

	onMount(() => {
		const interval = setInterval(() => {
			index = (index + 1) % slides.length;
			container.scrollTo({
				left: index * container.clientWidth,
				behavior: 'smooth'
			});
		}, 5000);

		return () => clearInterval(interval);
	});

	function getPositionClass(position: string) {
		const positions: { [key: string]: string } = {
			'bottom-center': 'bottom-4 left-1/2 -translate-x-1/2',
			'top-left': 'top-4 left-4',
			'bottom-right': 'bottom-4 right-4'
		};
		return positions[position] || positions['bottom-center'];
	}
</script>

<div bind:this={container} class="relative h-full overflow-hidden whitespace-nowrap">
	<div class="flex h-full">
		{#each slides as slide}
			<div class="relative w-full h-full flex-shrink-0">
				<img class="w-full h-full object-cover" src={slide.image.src} alt={slide.image.alt} />
				<div class={`absolute ${getPositionClass(slide.descriptionPosition)} p-4`}>
					<div class="bg-black/50 p-4 rounded-lg backdrop-blur-sm text-white">
						<Markdown source={slide.description} />
					</div>
				</div>
			</div>
		{/each}
	</div>
</div>
