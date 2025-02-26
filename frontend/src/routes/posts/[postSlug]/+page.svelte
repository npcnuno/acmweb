<script lang="ts">
	import type { Post } from '$lib/components/types';

	/** @type {import('./$types').PageData} */
	export let data;

	let post: Post = data.post;
</script>

<p>
	{post.title}
</p>
<p>
	{post.author}
</p>
<p>
	{post.content}
</p>
