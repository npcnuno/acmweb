<script lang="ts">

  export let change_menu: any;

</script>

<div id="previous-menu">

  <button id="previous-menu-button" on:click={() => change_menu( undefined ) } class="hover:bg-gray-900">

    <svg class="w-6 h-6 text-gray-800 dark:text-white" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
      <path stroke="white" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="m15 19-7-7 7-7"/>
    </svg>

  </button>

</div>

<style>

  #previous-menu {

    height: 50px;
    width: 100%;

    display: flex;
    align-items: center;
    padding: 10px;

  }

  #previous-menu-button {

    height: 40px;
    width: 40px;
    border-radius: 999px;
    display: flex;
    justify-content: center;
    align-items: center;

  }

</style>
