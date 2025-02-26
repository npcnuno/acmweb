<script lang="ts">
	import { onMount } from 'svelte';
	import { cn } from '../../../utils/cn';
	import { Motion } from 'svelte-motion';
	import BorderBeam from '../BorderBeam/BorderBeam.svelte';

	// Pass theme explicitly ('light' | 'dark') or leave as null to use system setting
	export let theme: 'light' | 'dark' | null = null;

	export let paragraphs: {
		text: string;
		words: { text: string; color?: string; join?: boolean }[];
		className?: string;
	}[] = [
		{
			text: 'main.java',
			words: [{ text: 'main.java', color: '#bd93f9' }],
			className: 'filename'
		},
		{
			text: '',
			words: [{ text: '', color: 'inherit' }]
		},
		{
			text: 'This is a sample Java program demonstrating polymorphism.',
			words: [
				{
					text: 'This is a sample Java program demonstrating polymorphism.',
					color: '#f8f8f2'
				}
			]
		},
		{
			text: 'It uses the Liskov Substitution Principle to show dynamic method binding.',
			words: [
				{
					text: 'It uses the Liskov Substitution Principle to show dynamic method binding.',
					color: '#f8f8f2'
				}
			]
		},
		{
			text: 'public class Main{',
			words: [
				{ text: 'public', color: '#ff79c6' },
				{ text: ' class', color: '#ff79c6' },
				{ text: ' Main', color: '#f1fa8c' },
				{ text: '{', color: '#f8f8f2' }
			]
		},
		{
			text: '    public static void main(String[] args) {',
			words: [
				{ text: '    ', color: 'inherit' },
				{ text: 'public', color: '#ff79c6' },
				{ text: ' static', color: '#ff79c6' },
				{ text: ' void', color: '#8be9fd' },
				{ text: ' main', color: '#8be9fd' },
				{ text: '(', color: '#f8f8f2', join: true },
				{ text: 'String[]', color: '#8be9fd' },
				{ text: ' args', color: '#f8f8f2', join: true },
				{ text: ')', color: '#f8f8f2' },
				{ text: ' {', color: '#f8f8f2' }
			]
		},
		{
			text: '        Animal animal = new Dog();',
			words: [
				{ text: '        ', color: 'inherit' },
				{ text: 'Animal', color: '#f1fa8c' },
				{ text: ' animal', color: '#f8f8f2' },
				{ text: ' =', color: '#f8f8f2' },
				{ text: ' new', color: '#ff79c6' },
				{ text: '  Dog', color: '#f1fa8c' },
				{ text: '();', color: '#f8f8f2' }
			]
		},
		{
			text: '        animal.makeSound(); // LSP in action',
			words: [
				{ text: '        ', color: 'inherit' },
				{ text: 'animal', color: '#f8f8f2', join: true },
				{ text: '.', color: '#f8f8f2', join: true },
				{ text: 'makeSound', color: '#8be9fd', join: true },
				{ text: '();', color: '#f8f8f2' },
				{ text: ' //', color: '#6272a4', join: true },
				{ text: ' LSP', color: '#ffb86c' },
				{ text: ' in', color: '#6272a4' },
				{ text: ' action', color: '#6272a4' }
			]
		},
		{
			text: '    }',
			words: [
				{ text: '    ', color: 'inherit' },
				{ text: '}', color: '#f8f8f2' }
			]
		},
		{
			text: '}',
			words: [{ text: '}', color: '#f8f8f2' }]
		},
		{
			text: '$ java Main',
			words: [
				{ text: '$', color: '#50fa7b' },
				{ text: ' java', color: '#8be9fd' },
				{ text: ' Main', color: '#f1fa8c' }
			],
			className: 'console-line'
		},
		{
			text: 'Animal makes sound: Woof!',
			words: [
				{ text: ' Animal', color: '#f1fa8c' },
				{ text: ' makes', color: '#f8f8f2' },
				{ text: ' sound:', color: '#f8f8f2' },
				{ text: ' Woof!', color: '#50fa7b' }
			],
			className: 'console-line'
		}
	];
	export let className: string | undefined = undefined;
	export let cursorClassName: string | undefined = undefined;
	export let scale: number = 1; // Scale for font size and spacing

	// effectiveTheme will be 'light' or 'dark'
	let effectiveTheme: 'light' | 'dark' = 'light';
	if (theme !== null) {
		effectiveTheme = theme;
	}

	// Reactively update effectiveTheme when the theme prop changes.
	$: {
		if (theme !== null) {
			effectiveTheme = theme;
		}
	}

	// If no theme is provided, use system preference and update on changes.
	onMount(() => {
		if (theme === null) {
			const darkModeMediaQuery = window.matchMedia('(prefers-color-scheme: dark)');
			effectiveTheme = darkModeMediaQuery.matches ? 'dark' : 'light';
			darkModeMediaQuery.addEventListener('change', (e) => {
				effectiveTheme = e.matches ? 'dark' : 'light';
			});
		}
	});

	let displayedWordsPerParagraph: string[][] = [];
	let isAnimating = false;
	let totalWordsAnimated = 0;
	let cursorPosition: { paraIndex: number; wordIndex: number } | null = null;
	let containerWidth = '0px';
	let containerHeight = '0px';

	$: if (paragraphs.length && !isAnimating) {
		isAnimating = true;
		totalWordsAnimated = 0;
		cursorPosition = { paraIndex: 0, wordIndex: 0 };
		animateParagraphs();
	}

	
	$: calculateContainerDimensions();

	function calculateContainerDimensions() {
		const baseFontSize = 14 * scale;
		const lineHeight = 1.5 * baseFontSize;
		const paragraphSpacing = 0.5 * baseFontSize;
		const indentationWidth = 4 * baseFontSize * 0.6;

		let maxLineLength = 0;
		let totalHeight = 0;
		paragraphs.forEach((paragraph) => {
			const lineLength = paragraph.text.length;
			maxLineLength = Math.max(maxLineLength, lineLength);
			totalHeight += lineHeight + paragraphSpacing;
		});

		const characterWidth = baseFontSize * 0.6;
		const linkHeight = 2 * baseFontSize; // space for the `<a>` element
		containerWidth = `${maxLineLength * characterWidth + indentationWidth + 40}px`;
		containerHeight = `${totalHeight + linkHeight + 40}px`;
	}

	async function animateParagraphs() {
		displayedWordsPerParagraph = paragraphs.map(() => []);

		for (let paraIndex = 0; paraIndex < paragraphs.length; paraIndex++) {
			const paragraph = paragraphs[paraIndex];
			const words = paragraph.words;

			for (const [wordIndex, word] of words.entries()) {
				const isLastWord = wordIndex === words.length - 1;
				const wordWithSpace = isLastWord ? word.text : word.text + ' ';

				displayedWordsPerParagraph[paraIndex] = [
					...displayedWordsPerParagraph[paraIndex],
					wordWithSpace
				];

				cursorPosition = { paraIndex, wordIndex };
				displayedWordsPerParagraph = displayedWordsPerParagraph;
				totalWordsAnimated++;

				await new Promise((resolve) => setTimeout(resolve, 300));
			}

			await new Promise((resolve) => setTimeout(resolve, 800));
		}

		isAnimating = false;
		cursorPosition = null;
	}
</script>

<!--
	Add the effectiveTheme class ('light' or 'dark') to the container.
	The CSS will update the background, border, and cursor colors based on this class.
-->
<div
	class={cn('relative code-container', className, effectiveTheme)}
	style="width: {containerWidth}; height: {containerHeight}; padding: 20px; overflow: hidden; border-radius: 12px;"
>
	<BorderBeam duration={4} size={500} />
	<div
		class="overflow-hidden text-justify break-words whitespace-pre-wrap font-bold"
		style="word-wrap: break-word; font-size: {14 * scale}px; line-height: {1.5 * scale};"
	>
		{#each displayedWordsPerParagraph as words, paraIndex (paraIndex)}
			<p
				class={cn('mb-4 paragraph-' + paraIndex, paragraphs[paraIndex]?.className)}
				style="margin-bottom: {0.5 * scale}rem;"
			>
				{#each words as word, wordIndex (wordIndex)}
					<span
						class={cn('inline-block word-' + wordIndex)}
						style="animation: fadeIn 0.1s ease-out; animation-delay: {(totalWordsAnimated -
							words.length +
							wordIndex) *
							0.05}s; color: {paragraphs[paraIndex]?.words[wordIndex]?.color || 'inherit'};"
					>
						{paragraphs[paraIndex]?.words[wordIndex]?.text}
					</span>
				{/each}

				{#if isAnimating && cursorPosition?.paraIndex === paraIndex}
					<Motion
						let:motion
						initial={{ opacity: 1 }}
						animate={{ opacity: 0 }}
						transition={{
							duration: 1,
							repeat: Infinity,
							repeatType: 'reverse'
						}}
					>
						<span
							use:motion
							class={cn('cursor', cursorClassName)}
							style="
								display: inline-block;
								width: {0.4 * scale}em;
								height: {1.2 * scale}em;
								vertical-align: bottom;
							"
						/>
					</Motion>
				{/if}
			</p>
		{/each}
	</div>
</div>

<style>
	/* Light theme variables */
	.code-container.light {
		--code-container-bg: #c0c0c0;
		--code-container-border: #333;
		--cursor-bg: #61afef;
		--link-color: #ddd;
		--link-border: #ddd;
		--link-hover-bg: #61afef;
		--link-hover-color: #fff;
	}

	/* Dark theme variables (note the updated cursor color) */
	.code-container.dark {
		--code-container-bg: #1e1e1e;
		--code-container-border: #444;
		--cursor-bg: #ff79c6;
		--link-color: #eee;
		--link-border: #eee;
		--link-hover-bg: #ff79c6;
		--link-hover-color: #fff;
	}

	.code-container {
		font-family: 'Fira Code', 'Menlo', 'Consolas', monospace;
		background-color: var(--code-container-bg);
		border: 1px solid var(--code-container-border);
		box-shadow: 0 4px 10px rgba(0, 0, 0, 0.3);
		transition:
			background-color 0.3s ease,
			border-color 0.3s ease;
	}

	.cursor {
		background-color: var(--cursor-bg);
		border-radius: 2px;
		transition: background-color 0.3s ease;
	}

	@keyframes fadeIn {
		from {
			opacity: 0;
			transform: translateY(-5px);
		}
		to {
			opacity: 1;
			transform: translateY(0);
		}
	}
</style>
