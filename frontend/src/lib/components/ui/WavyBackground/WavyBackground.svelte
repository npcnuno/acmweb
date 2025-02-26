<script lang="ts">
	import { cn } from '../../../utils/cn';
	import { createNoise3D } from 'simplex-noise';
	import { onMount } from 'svelte';

	export let className: string | undefined = undefined;
	export let colors: string[] | undefined = undefined;
	export let waveWidth: number | undefined = undefined;
	export let backgroundFill: string | undefined = undefined;
	export let blur: number | undefined = 5;
	export let speed: 'slow' | 'fast' | undefined = 'fast';
	export let waveOpacity: number | undefined = 0.5;

	const noise = createNoise3D();
	let w: number, h: number, nt: number, i: number, x: number;
	let ctx: CanvasRenderingContext2D;
	let canvasRef: HTMLCanvasElement;

	// Detect dark mode based on 'dark' class on <html>
	let isDarkMode = true;
	onMount(() => {
		const html = document.documentElement;
		isDarkMode = html.classList.contains('dark');
		const observer = new MutationObserver(() => {
			isDarkMode = html.classList.contains('dark');
		});
		observer.observe(html, { attributes: true, attributeFilter: ['class'] });
		init();
		return () => {
			observer.disconnect();
			cancelAnimationFrame(animationId);
		};
	});

	$: computedBackgroundFill = backgroundFill ?? (isDarkMode ? '#1a1a1a' : '#f0f0f0');
$: computedWaveColors = colors ?? (isDarkMode ? 
    ['rgb(2,137,188)', 'rgb(4,163,224)', 'rgb(6,189,255)', 'rgb(9,215,255)', 'rgb(12,241,255)'] : 
    ['rgb(0,85,116)', 'rgb(1,111,152)', 'rgb(2,137,188)', 'rgb(4,163,224)', 'rgb(6,189,255)']);

	const getSpeed = () => (speed === 'slow' ? 0.001 : 0.002);

	const init = () => {
		const canvas = canvasRef;
		ctx = canvas.getContext('2d')!;
		w = canvas.width = window.innerWidth;
		h = canvas.height = window.innerHeight;
		ctx.filter = `blur(${blur}px)`;
		nt = 0;
		window.onresize = () => {
			w = canvas.width = window.innerWidth;
			h = canvas.height = window.innerHeight;
			ctx.filter = `blur(${blur}px)`;
		};
		render();
	};

	const drawWave = (n: number) => {
		nt += getSpeed();
		for (i = 0; i < n; i++) {
			ctx.beginPath();
			ctx.lineWidth = waveWidth || 50;
			ctx.strokeStyle = computedWaveColors[i % computedWaveColors.length];
			for (x = 0; x < w; x += 5) {
				const y = noise(x / 800, 0.3 * i, nt) * 100;
				ctx.lineTo(x, y + h * 0.5);
			}
			ctx.stroke();
			ctx.closePath();
		}
	};

	let animationId: number;
const render = () => {
  ctx.globalAlpha = 1;
  ctx.fillStyle = computedBackgroundFill;
  ctx.fillRect(0, 0, w, h);
  drawWave(5);
  animationId = requestAnimationFrame(render);
};
</script>

<canvas class={className} bind:this={canvasRef} {...$$restProps}></canvas>
