<script lang="ts">

  import { change_page } from "../../../routes/(auth)/+layout.svelte";
  import { selected_button_url } from "./store";
  import "../../../app.pcss" 

</script>

<div id="user-info">

  <div id="background"></div>

  <div aria-hidden="true" on:click={ () => { selected_button_url.set( '/profile' ); change_page( '/profile' ); } } id="user-photo-name" class=" flex justify-center items-center flex-col cursor-pointer">

    <div id="user-photo" class="rounded-full"></div>

    <div id="name" class="text-white font-bold text-lg mt-5">Marinho Vieira</div>

  </div>

</div>

  

<style>

  #background {

    width: var( --side-bar-width );
    height: var( --side-bar-user-photo-background-height );
    background-color: red;
    background-image: url('$lib/assets/login_bg.jpg'); 
    background-repeat: no-repeat; 
    background-size: cover; 
    filter: blur(1px);

  }

  #user-info {
  
    width: var( --side-bar-width );
    height: var( --side-bar-user-photo-background-height );

  }

  #user-photo-name {

    width: var( --side-bar-width );
    height: var( --side-bar-user-photo-background-height );
    top: 0; left: 0;
    position: absolute;

  }

  #user-photo {

    width: var( --side-bar-user-photo-size );
    height: var( --side-bar-user-photo-size );
    background-color: red;

    background-image: url('$lib/assets/user_photo.jpg'); 
    background-repeat: no-repeat; 
    background-size: cover; 


  }

  #name {

    display: var( --side-bar-normal-screen-display );

  }

</style>
