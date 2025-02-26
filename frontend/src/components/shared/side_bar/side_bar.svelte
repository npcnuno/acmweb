<script lang="ts">
  /** IMPORTS **/
  import "../../../app.pcss";
  import { onMount } from "svelte";
  import Button from "./button.svelte";
  import Footer from "./footer.svelte";
  import Separator from "./separator.svelte";
  import UserPhoto from "./user_photo.svelte";
  import DropSideButton from "./drop_side_button.svelte";
  import { main_menu, event_sub_menu } from "./menus_data";
  import { on_resize } from "./listenners";
  import PreviousMenuButton from "./previous_menu_button.svelte";
  import { page } from "$app/stores";
  import { event_editing_store } from "$lib/stores/globalStore";

  /** DECLARATIONS **/

  // Stack of current menus in use for the ui
  // Acts like a stack so the top menu options are the one in use
  let current_menu = [main_menu];

  $: {
    const path = $page.url.pathname;
    $event_editing_store.sub_events.length; // Reactively subscribe to the store

    if (path.startsWith("/management/events/")) {
      current_menu = [event_sub_menu]; // Always use event_sub_menu for this path
    } else {
      current_menu = [main_menu]; // Use main_menu for other paths
    }
  }

  async function change_menu(new_menu?: []) {
    // Get the element menu for the animation
    const menu_element = document.getElementById("menu");

    if (menu_element === null || current_menu === undefined) return;

    // Animation fade away
    menu_element.style["opacity"] = "0";

    // Wait for the animation to finish
    await new Promise((resolve) => setTimeout(resolve, 200));

    // If new array is given push it into the current_menu array and update the menu display stack
    if (new_menu !== undefined) current_menu = [...current_menu, new_menu];
    // If no new_array is given pop the last one in stack
    else {
      current_menu.pop();
      current_menu = current_menu;
    }

    // Execute the animation to fade back to normal
    menu_element.style["opacity"] = "1";
  }

  onMount(() => {
    // Setup listenners
    document
      .getElementById("nav-bar")
      ?.addEventListener("mouseover", (event) =>
        event.stopImmediatePropagation(),
      );
    document
      .getElementById("toggle-side-bar")
      ?.addEventListener("click", () => on_resize(undefined, true));
    document
      .getElementById("side-bar")
      ?.addEventListener("mouseover", () => on_resize(undefined, true));
    document
      .getElementById("side-bar")
      ?.addEventListener("mouseout", () => on_resize());
    window.onresize = on_resize;
    on_resize();
  });
</script>

<div id="side-bar" class="flex flex-col fixed">
  <div id="nav-bar" class="justify-start items-center cursor-pointer fixed">
    <svg
      id="toggle-side-bar"
      class="w-7 h-7 text-gray-800 dark:text-white m-5"
      aria-hidden="true"
      xmlns="http://www.w3.org/2000/svg"
      fill="none"
      viewBox="0 0 24 24"
    >
      <path
        stroke="white"
        stroke-linecap="round"
        stroke-width="2"
        d="M5 7h14M5 12h14M5 17h14"
      />
    </svg>
  </div>

  <UserPhoto />

  <div id="menu" class="grow overflow-y-auto mb-[60px] transition">
    {#each current_menu[current_menu.length - 1] as { type, name, icon, sub_menu, redirect } (type + name + redirect ?? "")}
      {#if type == "Previous_Menu"}
        <PreviousMenuButton {change_menu} />
      {:else if type == "Separator"}
        <Separator {name} />
      {:else if type == "Button" && icon !== undefined && redirect !== undefined}
        <Button {name} {icon} {redirect} />
      {:else if type == "Drop_Side" && icon !== undefined && sub_menu !== undefined}
        <DropSideButton {name} {icon} {change_menu} {sub_menu} />
      {/if}
    {/each}
  </div>

  <Footer />
</div>

<style>
  #side-bar {
    /* VARIABLES */

    /* SIDE BAR GENERAL */
    --side-bar-width: 330px;
    --side-bar-display-flex: flex;
    --side-bar-normal-screen-display: block;
    --side-bar-normal-screen-display-flex: flex;
    --side-bar-small-screen-display: none;
    --side-bar-phone-screen-display-flex: none;

    /* USER PHOTO */
    --side-bar-user-photo-background-height: 250px;
    --side-bar-user-photo-size: 140px;

    /* BUTTON */
    --side-bar-button-icon-margin: 20px;

    /* FOOTER */
    --side-bar-footer-height: 60px;
    --side-bar-footer-button-sizes: 45px;
    --side-bar-footer-notifications-count-sizes: 15px;
    --side-bar-footer-notifications-count-left: 245px;
    --side-bar-footer-notifications-popup-width: 275px;
    --side-bar-footer-notifications-popup-height: 175px;
    --side-bar-footer-notifications-popup-margin-left: 0px;
    --side-bar-footer-notifications-popup-display: none;

    width: var(--side-bar-width);
    height: 100svh;

    background-color: var(--secondary-color);
  }

  #nav-bar {
    width: 100svw;
    height: 50px;
    display: var(--side-bar-phone-screen-display-flex);
    background-color: var(--secondary-color);
  }
</style>
