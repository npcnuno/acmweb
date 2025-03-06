<script lang="ts">
  import { toggleMode } from "mode-watcher";
</script>

<button
  on:click={toggleMode}
  class="relative flex items-center justify-center px-2 py-2 rounded-md text-sm hover:-translate-y-1 transform transition duration-200 hover:shadow-md"
>
  <img
    width="100"
    class="h-5 w-5 z-0 rotate-0 scale-100 transition-all dark:-rotate-90 dark:scale-0"
    height="100"
    src="https://img.icons8.com/ios/100/sun--v1.png"
    alt="sun--v1"
  />

  <img
    class="absolute vertical-align-middle item-center h-5 w-5 rotate-90 scale-0 transition-all dark:rotate-0 dark:scale-100"
    width="100"
    height="100"
    src="moon.png"
    alt="moon-satellite"
  />
</button>
