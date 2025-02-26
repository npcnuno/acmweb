<script lang="ts">

  import "../../../app.pcss"

  export let name: string;

  export let icon: string;

  export let change_menu: any;

  export let sub_menu: {}[];

</script>

<div id="button" aria-hidden="true" on:click={ () => { change_menu( sub_menu ) } } class="flex justify-start items-center rounded-md hover:bg-gray-900 cursor-pointer">

  { @html icon }

  <div id="name" class="text-white font-semibold">{ name }</div>

</div>

<style>

  #button {

    height: 60px;
    margin: 10px;

  }

  #name {

    display: var( --side-bar-normal-screen-display );

  }

</style>
