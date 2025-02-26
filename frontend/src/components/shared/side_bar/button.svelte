<script lang="ts">

  import { change_page } from './../../../routes/(auth)/+layout.svelte'
  import { goto } from "$app/navigation";
  import { onMount } from "svelte";
  import "../../../app.pcss"
  import { selected_button_url } from './store';

  export let name: string;
  export let icon: string;
  export let redirect: string;
  
  var base_button_class = 'flex justify-start items-center rounded-md hover:bg-gray-900 cursor-pointer';
  let button_class = "";

  onMount( () => {

    selected_button_url.subscribe(

      ( value ) => {

        button_class = base_button_class;

        if ( value == '' && window.location.pathname == redirect || value == redirect ) 
          
          button_class += " bg-gray-900";

      }

    );

  }

  );

</script>

<div id="button" aria-hidden="true" on:click={ () => { selected_button_url.set( redirect ); change_page( redirect ); } } class={ button_class }>

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
