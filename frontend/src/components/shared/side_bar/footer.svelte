<script lang="ts">
  
  import { onMount } from "svelte";
  import "../../../app.pcss"

  /* VARIABLES */
  let notifications_popup_open = false;

  onMount(

    () => {
  
      document.getElementById( 'notifications-popup' )?.addEventListener(

        'mousedown',

        (e) => { e.stopImmediatePropagation(); }

      );

      document.getElementById( 'notifications-popup' )?.addEventListener(

        'focusout',

        (e) => {
          
          document.querySelector('#side-bar').style.setProperty( '--side-bar-footer-notifications-popup-display', 'none' );
          notifications_popup_open = false;

        }

      );
      
      document.getElementById( 'notifications-toggle' )?.addEventListener(
  
        'mousedown',

        (e) => {

          e.preventDefault();

          document.querySelector('#side-bar').style.setProperty( '--side-bar-footer-notifications-popup-display', notifications_popup_open ? 'none' : 'block' );

          notifications_popup_open = !notifications_popup_open;

          document.getElementById( 'notifications-popup' )?.focus();

        }

      );

    }

  )

</script>

<div id="side-bar-footer" class="fixed justify-center items-center text-white font-bold bottom-0 pl-2 pr-2" >

  <div class="m-2" style="display: var( --side-bar-normal-screen-display);">AeWallet</div>

  <div class="grow"style="display: var( --side-bar-normal-screen-display);"></div>

  <button id="notifications-toggle" tabindex="0" class="footer-buttons cursor-pointer justify-center items-center">

    <svg class="w-7 h-7 text-gray-800 dark:text-white" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="white" viewBox="0 0 24 24">
      <path fill-rule="evenodd" d="M3.6 4.5c.3-.3.8-.5 1.3-.5H19a1.9 1.9 0 0 1 2 1.9V15a1.9 1.9 0 0 1-1.9 1.9h-3.6l-2.7 3.6a1 1 0 0 1-1.6 0L8.5 17H4.9A1.9 1.9 0 0 1 3 15.1V6c0-.5.2-1 .6-1.4Zm4 3a1 1 0 1 0 0 2h8a1 1 0 0 0 0-2h-8Zm0 3.5a1 1 0 1 0 0 2H12a1 1 0 1 0 0-2H7.6Z" clip-rule="evenodd"/>
    </svg>

    <div id="notifications-count" class="absolute rounded-full flex justify-center items-center text-xs">6</div>

    <div id="notifications-popup" tabindex="-1" class="absolute rounded-md shadow cursor-default outline-none"></div>

  </button>

  <button id='logout' tabindex="0" class="footer-buttons flex justify-center cursor-pointer items-center">

    <svg class="w-7 h-7 text-gray-800 dark:text-white transition-all" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
      <path stroke="white" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 12H4m12 0-4 4m4-4-4-4m3-4h2a3 3 0 0 1 3 3v10a3 3 0 0 1-3 3h-2"/>
    </svg>
  
  </button>

</div>

<style>

  #side-bar-footer {

    width: var( --side-bar-width );
    display: var( --side-bar-display-flex );
    height: var( --side-bar-footer-height );

    background-color: var( --secondary-color-strong );

  }

  .footer-buttons {
  
    margin: 5px;
    height: var( --side-bar-footer-button-sizes );
    width: var( --side-bar-footer-button-sizes );

  }

  #notifications-toggle {

    display: var( --side-bar-display-flex );

  }

  #notifications-count {

    height: var( --side-bar-footer-notifications-count-sizes );
    width: var( --side-bar-footer-notifications-count-sizes );
    bottom: 32px;
    left: var( --side-bar-footer-notifications-count-left );

    background-color: red;
  
  }

  #notifications-popup {

    height: var( --side-bar-footer-notifications-popup-height );
    width: var( --side-bar-footer-notifications-popup-width );
    bottom: 65px;
    display: var( --side-bar-footer-notifications-popup-display );
    margin-left: var( --side-bar-footer-notifications-popup-margin-left );

    background-color: white;

  }

  #logout {

    display: var( --side-bar-normal-screen-display-flex );

  }

</style>
