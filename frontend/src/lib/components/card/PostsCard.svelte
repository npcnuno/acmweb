<script lang="ts">
  export let data;
</script>

<a
  href={`/posts/${data.id}`}
  title={`Read more about ${data.title}`}
  class="block group"
>
  <div
    class="flex w-full flex-col overflow-hidden rounded-xl border border-gray-200 bg-white text-gray-700 shadow-sm transition-shadow duration-200 hover:shadow-md dark:bg-gray-800 dark:border-gray-700"
  >
    <div class="relative overflow-hidden">
      <img
        src={data.img}
        alt={data.img_alt}
        class="h-48 w-full object-cover transition-transform duration-200 group-hover:scale-105"
      />
    </div>
    <div class="p-4">
      <h4 class="text-xl font-semibold text-blue-600 dark:text-blue-400 mb-2">
        {data.title}
      </h4>
      <p class="mt-2 text-sm text-gray-600 dark:text-gray-300 line-clamp-3">
        {data.description}
      </p>
    </div>
    <div class="flex items-center justify-between px-4 py-3">
      <div class="flex items-center space-x-2">
        <img
          src={data.author.img}
          alt={data.author.name}
          class="h-8 w-8 rounded-full object-cover"
        />
        <span class="text-sm font-medium text-gray-700 dark:text-gray-200">
          {data.author.name}
        </span>
      </div>
      <p class="text-sm text-gray-500 dark:text-gray-400">{data.date}</p>
    </div>
  </div>
</a>
