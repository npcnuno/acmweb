<script lang="ts">
  import PostsCard from "../../lib/components/card/PostsCard.svelte";

  let posts = [
    {
      id: 1,
      title: "NOVO PRESIDENTE DA ACM",
      img: "https://images.unsplash.com/photo-1522202176988-66273c2fd55f?...",
      img_alt: "YES YES YES",
      description: "Because it’s about motivating the doers...",
      author: {
        img: "https://images.unsplash.com/photo-1633332755192-727a05c4013d?...",
        name: "KATCHOW",
      },
      date: "January 30th",
    },
    // Add more posts...
  ];
</script>

<div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6 p-6">
  {#each posts as post}
    <PostsCard data={post} />
  {/each}
</div>
