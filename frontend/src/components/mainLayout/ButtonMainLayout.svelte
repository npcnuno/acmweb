<script lang="ts">
  export let path: string;
  export let buttonName: string;
</script>

<a class="group text-white transition-all duration-300 ease-in-out" href={path}>
  <span
    class="bg-left-bottom text-gray-800 dark:text-white bg-gradient-to-r from-gray-900 to-gray-600 dark:bg-gradient-to-r dark:from-blue-100 dark:to-blue-800 bg-[length:0%_2px] bg-no-repeat group-hover:bg-[length:100%_2px] transition-all duration-500 ease-out"
  >
    {buttonName}
  </span>
</a>
