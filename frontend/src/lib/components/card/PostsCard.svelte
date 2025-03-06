<script lang="ts">
  export let data;
</script>

<a
  href={`/posts/${data.id}`}
  title={`Read more about ${data.title}`}
  class="block group"
>
  <div
    class="rounded-lg border border-gray-200 bg-white shadow-md hover:shadow-xl transition-shadow duration-300 dark:bg-gray-800 dark:border-gray-700"
  >
    <!-- Header with title and optional language badge -->
    <div class="p-4">
      <div class="flex items-center justify-between">
        <h2 class="text-2xl font-semibold text-gray-800 dark:text-gray-100">
          {data.title}
        </h2>
        {#if data.lang}
          <span
            class="px-2 py-1 text-xs font-medium text-blue-600 bg-blue-100 rounded dark:text-blue-100 dark:bg-blue-600"
          >
            {data.lang}
          </span>
        {/if}
      </div>
      <p class="mt-2 text-gray-600 dark:text-gray-300">
        {data.description}
      </p>
    </div>

    <!-- Footer with author info and post date -->
    <div
      class="flex items-center justify-between border-t border-gray-200 p-4 dark:border-gray-700"
    >
      <div class="flex items-center">
        {#if data.author && data.author.img}
          <img
            src={data.author.img}
            alt={data.author.name}
            class="h-10 w-10 rounded-full object-cover"
          />
        {:else}
          <!-- Placeholder avatar using the first letter of the author's name -->
          <div
            class="h-10 w-10 rounded-full bg-gray-300 flex items-center justify-center text-gray-600"
          >
            {data.author.name ? data.author.name.charAt(0) : "A"}
          </div>
        {/if}
        <div class="ml-3">
          <p class="text-sm font-medium text-gray-800 dark:text-gray-100">
            {data.author.name}
          </p>
          <p class="text-xs text-gray-500 dark:text-gray-400">
            {data.date}
          </p>
        </div>
      </div>
      <!-- Optional "read more" arrow icon -->
      <svg
        class="h-5 w-5 text-gray-400 group-hover:text-gray-600 dark:text-gray-500 dark:group-hover:text-gray-300"
        fill="none"
        stroke="currentColor"
        viewBox="0 0 24 24"
        xmlns="http://www.w3.org/2000/svg"
      >
        <path
          stroke-linecap="round"
          stroke-linejoin="round"
          stroke-width="2"
          d="M9 5l7 7-7 7"
        ></path>
      </svg>
    </div>
  </div>
</a>
