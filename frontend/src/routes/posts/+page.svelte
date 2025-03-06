<script lang="ts">
  import { onMount } from "svelte";
  import PostsCard from "../../lib/components/card/PostsCard.svelte";
  import { postsStore } from "../../stores/globalStores";
  import { get_posts } from "wasm-test"; // adjust the import path as needed
  import init from "wasm-test";

  // Load posts when the component mounts.
  onMount(async () => {
    try {
      await init(); // Wait for WASM initialization.
      const response = await get_posts("en");
      console.log(response);
      postsStore.set(response);
    } catch (error) {
      console.error("getPosts failed", error);
    }
  });
</script>

<div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6 p-6">
  {#each $postsStore as post (post.id)}
    <PostsCard data={post} />
  {/each}
</div>
