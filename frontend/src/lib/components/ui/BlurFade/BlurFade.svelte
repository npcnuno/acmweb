<script lang="ts">
	import { cn } from '../../../utils/cn';
	import { Motion } from 'svelte-motion';

	export let word: string = 'Blur In';
	export let variant: {
		hidden: { filter: string; opacity: number };
		visible: { filter: string; opacity: number };
	} = {
		hidden: { filter: 'blur(10px)', opacity: 0 },
		visible: { filter: 'blur(0px)', opacity: 1 }
	};
	export let duration: number = 1;
	let className = '';
	export { className as class };

	let defaultVariants = {
		hidden: { filter: 'blur(10px)', opacity: 0 },
		visible: { filter: 'blur(0px)', opacity: 1 }
	};
	let combinedVariants = variant || defaultVariants;
</script>

<Motion
	initial="hidden"
	animate="visible"
	transition={{ duration }}
	variants={combinedVariants}
	let:motion
>
	<span
		class={cn(className, 'text-gray-900 whitespace-nowrap font-bold drop-shadow-sm ')}
		use:motion
	>
		{word}
	</span>
</Motion>
