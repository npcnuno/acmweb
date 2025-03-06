<script lang="ts">
  import "../app.postcss";
  import { AppBar, storePopup } from "@skeletonlabs/skeleton";
  import { ModeWatcher } from "mode-watcher";
  import {
    computePosition,
    autoUpdate,
    flip,
    shift,
    offset,
    arrow,
  } from "@floating-ui/dom";
  import { WavyBackground } from "../lib/components/ui/WavyBackground";
  import ButtonMainLayout from "../components/mainLayout/ButtonMainLayout.svelte";
  import DarkmodeSwitch from "../lib/components/ui/darkmodeSwitch/DarkmodeSwitch.svelte";
  import { fly, fade } from "svelte/transition";
  import { t, locale } from "svelte-i18n";
  import { alertMessage } from "../stores/alertStore"; // import the alert store
  import { onMount } from "svelte";
  import init from "wasm-test";
  import { getPosts } from "../lib/services/service";
  // Setup Floating UI for popups
  storePopup.set({ computePosition, autoUpdate, flip, shift, offset, arrow });

  let menuOpen = false;
  function toggleMenu() {
    menuOpen = !menuOpen;
  }
  onMount(async () => {
    await init();
    try {
      await getPosts("en");
    } catch (error) {
      console.error("Could not load initial posts:", error);
      // Continue initialization even if posts fail to load
    }
  });
  // Language options with proper display format
  const languageOptions = [
    { code: "en", flag: "🇺🇸", name: "English" },
    { code: "pt", flag: "🇵🇹", name: "Português" },
    { code: "de", flag: "🇩🇪", name: "Deutsch" },
    { code: "zh", flag: "🇨🇳", name: "中文" },
    { code: "fr", flag: "🇫🇷", name: "Français" },
    { code: "ja", flag: "🇯🇵", name: "日本語" },
  ];

  let showLanguageMenu = false;
  let languageSwitcher;

  // Reactively set the current language based on $locale
  $: currentLanguage = languageOptions.find(
    (option) => option.code === $locale,
  ) || { flag: "🌐", name: "Select Language" };

  // Toggle the language menu visibility
  function toggleLanguageMenu() {
    showLanguageMenu = !showLanguageMenu;
  }

  // Select a language and close the menu
  function selectLanguage(code) {
    $locale = code;
    showLanguageMenu = false;
  }

  // Close the menu when clicking outside
  onMount(() => {
    function handleClickOutside(event) {
      if (languageSwitcher && !languageSwitcher.contains(event.target)) {
        showLanguageMenu = false;
      }
    }
    document.addEventListener("click", handleClickOutside);
    return () => {
      document.removeEventListener("click", handleClickOutside);
    };
  });
</script>

<ModeWatcher />

<!-- ACM-themed Wavy Background -->
<WavyBackground
  class="fixed inset-0 z-0 pointer-events-none h-screen"
  colors={["#5B92E5", "#3B71B4", "#1E3F66", "#032B5F"]}
/>

<!-- Global Alert Popup -->
{#if $alertMessage}
  <div class="alert-popup" transition:fade>{{ $alertMessage }}</div>
{/if}

<!-- AppBar Navigation with improved ACM branding colors -->
<AppBar
  class="z-[1000] w-full fixed top-0 left-0 shadow-lg transition-all"
  gridColumns="grid-cols-3"
  slotDefault="place-self-center"
  slotTrail="place-content-end"
>
  <svelte:fragment slot="lead">
    <a
      href="/"
      class="flex items-center space-x-2 sm:space-x-3 hover:opacity-80 transition-opacity"
    >
      <img
        alt="ACM Chapter Logo"
        src="https://iscte.acm.org/wp-content/uploads/2015/04/cropped-LOGOv1-4.png"
        class="w-10 h-10 sm:w-12 sm:h-12 rounded-full p-1"
      />
      <div class="flex flex-col">
        <span
          class="text-grey-800 dark:text-white text-lg sm:text-xl font-bold tracking-tight"
          >ACM ISCTE</span
        >
        <span
          class="text-grey-800/70 dark:text-white/70 text-xs hidden sm:block"
          >Student Chapter</span
        >
      </div>
    </a>
  </svelte:fragment>

  <!-- Desktop Navigation -->
  <div class="hidden md:flex space-x-6 justify-center">
    <ButtonMainLayout path="/" buttonName={$t("nav.home")} />
    <ButtonMainLayout path="/projects" buttonName={$t("nav.projects")} />
    <ButtonMainLayout path="/posts" buttonName={$t("nav.posts")} />
    <ButtonMainLayout path="/about" buttonName={$t("nav.about")} />
  </div>

  <svelte:fragment slot="trail">
    <div class="flex items-center gap-2 sm:gap-3">
      <DarkmodeSwitch />

      <!-- Language Switcher Dropdown -->
      <div bind:this={languageSwitcher} class="relative max-sm:hidden">
        <button
          on:click={toggleLanguageMenu}
          aria-label={`Current language: ${currentLanguage.name}`}
          class="relative flex items-center justify-center rounded-md text-sm hover:-translate-y-1 transform transition duration-200 hover:shadow-md"
        >
          <span class="text-2xl">{currentLanguage.flag}</span>
        </button>
        {#if showLanguageMenu}
          <ul
            class="absolute top-full left-0 mt-1 py-1 bg-white border border-gray-300 rounded-md shadow-md"
          >
            {#each languageOptions as option}
              <li
                on:click={() => selectLanguage(option.code)}
                role="button"
                aria-label={`Select ${option.name}`}
                class="px-2 py-1 hover:bg-gray-100 cursor-pointer"
              >
                <span class="text-2xl">{option.flag}</span>
              </li>
            {/each}
          </ul>
        {/if}
      </div>

      <!-- Join/Register Button -->
      <a
        href="/register"
        class="bg-white text-blue-800 max-sm:hidden px-4 py-2 rounded-lg hover:bg-blue-50 transition-colors shadow-md font-medium"
      >
        {$t("nav.signup")}
      </a>

      <!-- Mobile Menu Toggle Button -->
      <button
        class="md:hidden flex items-center p-2 rounded-lg text-white hover:bg-blue-500/20 transition-colors"
        on:click={toggleMenu}
        aria-label="Toggle menu"
      >
        <svg class="fill-current h-6 w-6" viewBox="0 0 24 24">
          <path
            fill="currentColor"
            d="M3 18h18v-2H3v2zm0-5h18v-2H3v2zm0-7v2h18V6H3z"
          />
        </svg>
      </button>
    </div>
  </svelte:fragment>
</AppBar>

<!-- Mobile Menu with Animation -->
{#if menuOpen}
  <div
    class="fixed inset-0 bg-black/50 z-[998]"
    on:click={toggleMenu}
    transition:fade={{ duration: 200 }}
  ></div>

  <div
    class="md:hidden fixed inset-y-0 right-0 w-4/5 max-w-sm h-full bg-white dark:bg-gray-900 z-[999] flex flex-col p-6"
    transition:fly={{ x: 300, duration: 300 }}
  >
    <div class="flex justify-between items-center mb-8">
      <span class="text-xl font-bold text-blue-800 dark:text-blue-300"
        >Menu</span
      >
      <button
        on:click={toggleMenu}
        class="p-2 rounded-full hover:bg-gray-100 dark:hover:bg-gray-800"
      >
        <svg
          xmlns="http://www.w3.org/2000/svg"
          class="h-6 w-6"
          fill="none"
          viewBox="0 0 24 24"
          stroke="currentColor"
        >
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            stroke-width="2"
            d="M6 18L18 6M6 6l12 12"
          />
        </svg>
      </button>
    </div>

    <div class="flex flex-col space-y-4">
      <ButtonMainLayout
        path="/"
        buttonName={$t("nav.home")}
        on:click={toggleMenu}
      />
      <ButtonMainLayout
        path="/projects"
        buttonName={$t("nav.projects")}
        on:click={toggleMenu}
      />
      <ButtonMainLayout
        path="/events"
        buttonName={$t("nav.events")}
        on:click={toggleMenu}
      />

      <ButtonMainLayout
        path="/about"
        buttonName={$t("nav.about")}
        on:click={toggleMenu}
      />
    </div>

    <div class="mt-auto space-y-4">
      <!-- Join/Register Button for Mobile -->
      <a
        href="/register"
        class="block w-full bg-blue-800 text-white text-center py-3 rounded-lg hover:bg-blue-700 transition-colors font-medium"
      >
        {$t("nav.signup")}
      </a>

      <!-- Language Switcher for Mobile -->
      <div class="bg-gray-100 dark:bg-gray-800 rounded-lg p-4">
        <p class="text-sm text-gray-500 dark:text-gray-400 mb-2">
          {$t("nav.language")}
        </p>
        <div class="grid grid-cols-2 gap-2">
          {#each languageOptions as option}
            <button
              class="flex items-center space-x-2 p-2 rounded-md hover:bg-gray-200 dark:hover:bg-gray-700 {$locale ===
              option.code
                ? 'bg-blue-100 dark:bg-blue-900/30 font-medium'
                : ''}"
              on:click={() => {
                $locale = option.code;
              }}
            >
              <span class="text-lg">{option.flag}</span>
              <span class="text-sm">{option.name}</span>
            </button>
          {/each}
        </div>
      </div>
    </div>
  </div>
{/if}

<!-- Main Content Area - Added bottom padding for better spacing -->
<main
  class="relative z-10 mt-16 sm:mt-20 md:mt-24 px-4 sm:px-6 pb-16 max-w-7xl mx-auto"
>
  <slot />
</main>

<!-- Footer with improved colors -->
<footer
  class="relative z-10 bg-gradient-to-r from-blue-900 to-blue-800 text-white py-8"
>
  <div class="container mx-auto px-4 sm:px-6">
    <div class="grid grid-cols-1 md:grid-cols-3 gap-8">
      <!-- Logo & Description -->
      <div class="flex flex-col">
        <a href="/" class="flex items-center space-x-2 mb-4">
          <img
            alt="ACM Chapter Logo"
            src="https://iscte.acm.org/wp-content/uploads/2015/04/cropped-LOGOv1-4.png"
            class="w-8 h-8 sm:w-10 sm:h-10 rounded-full bg-white p-1"
          />
          <span class="text-lg sm:text-xl font-bold">ACM ISCTE</span>
        </a>
        <p class="text-sm text-blue-100">
          {$t("footer.description")}
        </p>
      </div>

      <!-- Quick Links -->
      <div>
        <h3 class="text-lg font-semibold mb-4">{$t("footer.links")}</h3>
        <ul class="space-y-2 grid grid-cols-2 sm:block">
          <li>
            <a
              href="/about"
              class="text-blue-200 hover:text-white transition-colors inline-block py-1"
              >{$t("footer.about")}</a
            >
          </li>
          <li>
            <a
              href="/events"
              class="text-blue-200 hover:text-white transition-colors inline-block py-1"
              >{$t("footer.events")}</a
            >
          </li>
          <li>
            <a
              href="/projects"
              class="text-blue-200 hover:text-white transition-colors inline-block py-1"
              >{$t("footer.projects")}</a
            >
          </li>
          <li>
            <a
              href="/join"
              class="text-blue-200 hover:text-white transition-colors inline-block py-1"
              >{$t("footer.join")}</a
            >
          </li>
        </ul>
      </div>

      <!-- Contact -->
      <div>
        <h3 class="text-lg font-semibold mb-4">{$t("footer.contact")}</h3>
        <ul class="space-y-2">
          <li class="flex items-center space-x-2">
            <svg
              xmlns="http://www.w3.org/2000/svg"
              class="h-5 w-5 flex-shrink-0"
              fill="none"
              viewBox="0 0 24 24"
              stroke="currentColor"
            >
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M3 8l7.89 5.26a2 2 0 002.22 0L21 8M5 19h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z"
              />
            </svg>
            <a
              href="mailto:contact@acmiscte.org"
              class="text-blue-200 hover:text-white transition-colors text-sm sm:text-base truncate"
              >contact@acmiscte.org</a
            >
          </li>
          <li class="flex items-center space-x-2">
            <svg
              xmlns="http://www.w3.org/2000/svg"
              class="h-5 w-5 flex-shrink-0"
              fill="none"
              viewBox="0 0 24 24"
              stroke="currentColor"
            >
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M17.657 16.657L13.414 20.9a1.998 1.998 0 01-2.827 0l-4.244-4.243a8 8 0 1111.314 0z"
              />
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M15 11a3 3 0 11-6 0 3 3 0 016 0z"
              />
            </svg>
            <span class="text-blue-200 text-sm sm:text-base"
              >ISCTE University, Lisbon</span
            >
          </li>
        </ul>
        <div class="mt-4 flex space-x-4">
          <a
            href="https://github.com/acm-iscte"
            class="text-blue-200 hover:text-white transition-colors"
            aria-label="GitHub"
          >
            <svg
              xmlns="http://www.w3.org/2000/svg"
              class="h-6 w-6"
              fill="currentColor"
              viewBox="0 0 24 24"
            >
              <path
                d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"
              />
            </svg>
          </a>
          <a
            href="https://twitter.com/acm_iscte"
            class="text-blue-200 hover:text-white transition-colors"
            aria-label="Twitter"
          >
            <svg
              xmlns="http://www.w3.org/2000/svg"
              class="h-6 w-6"
              fill="currentColor"
              viewBox="0 0 24 24"
            >
              <path
                d="M24 4.557c-.883.392-1.832.656-2.828.775 1.017-.609 1.798-1.574 2.165-2.724-.951.564-2.005.974-3.127 1.195-.897-.957-2.178-1.555-3.594-1.555-3.179 0-5.515 2.966-4.797 6.045-4.091-.205-7.719-2.165-10.148-5.144-1.29 2.213-.669 5.108 1.523 6.574-.806-.026-1.566-.247-2.229-.616-.054 2.281 1.581 4.415 3.949 4.89-.693.188-1.452.232-2.224.084.626 1.956 2.444 3.379 4.6 3.419-2.07 1.623-4.678 2.348-7.29 2.04 2.179 1.397 4.768 2.212 7.548 2.212 9.142 0 14.307-7.721 13.995-14.646.962-.695 1.797-1.562 2.457-2.549z"
              />
            </svg>
          </a>
          <a
            href="https://discord.gg/acmiscte"
            class="text-blue-200 hover:text-white transition-colors"
            aria-label="Discord"
          >
            <svg
              xmlns="http://www.w3.org/2000/svg"
              class="h-6 w-6"
              fill="currentColor"
              viewBox="0 0 24 24"
            >
              <path
                d="M20.317 4.3698a19.7913 19.7913 0 00-4.8851-1.5152.0741.0741 0 00-.0785.0371c-.211.3753-.4447.8648-.608 1.2495-1.8447-.2762-3.68-.2762-5.4868 0-.1634-.3933-.4058-.8742-.6177-1.2495a.077.077 0 00-.0785-.037 19.7363 19.7363 0 00-4.8852 1.515.0699.0699 0 00-.0321.0277C.5334 9.0458-.319 13.5799.0992 18.0578a.0824.0824 0 00.0312.0561c2.0528 1.5076 4.0413 2.4228 5.9929 3.0294a.0777.0777 0 00.0842-.0276c.4616-.6304.8731-1.2952 1.226-1.9942a.076.076 0 00-.0416-.1057c-.6528-.2476-1.2743-.5495-1.8722-.8923a.077.077 0 01-.0076-.1277c.1258-.0943.2517-.1923.3718-.2914a.0743.0743 0 01.0776-.0105c3.9278 1.7933 8.18 1.7933 12.0614 0a.0739.0739 0 01.0785.0095c.1202.099.246.1981.3728.2924a.077.077 0 01-.0066.1276 12.2986 12.2986 0 01-1.873.8914.0766.0766 0 00-.0407.1067c.3604.698.7719 1.3628 1.225 1.9932a.076.076 0 00.0842.0286c1.961-.6067 3.9495-1.5219 6.0023-3.0294a.077.077 0 00.0313-.0552c.5004-5.177-.8382-9.6739-3.5485-13.6604a.061.061 0 00-.0312-.0286zM8.02 15.3312c-1.1825 0-2.1569-1.0857-2.1569-2.419 0-1.3332.9555-2.4189 2.157-2.4189 1.2108 0 2.1757 1.0952 2.1568 2.419 0 1.3332-.9555 2.4189-2.1569 2.4189zm7.9748 0c-1.1825 0-2.1569-1.0857-2.1569-2.419 0-1.3332.9554-2.4189 2.1569-2.4189 1.2108 0 2.1757 1.0952 2.1568 2.419 0 1.3332-.946 2.4189-2.1568 2.4189Z"
              />
            </svg>
          </a>
        </div>
      </div>
    </div>

    <div
      class="mt-8 pt-6 border-t border-blue-700 text-center text-sm text-blue-300"
    >
      <p>
        © {new Date().getFullYear()} ACM ISCTE Student Chapter. {$t(
          "footer.rights",
        )}
      </p>
      <p class="mt-1">
        <a href="https://www.acm.org" class="hover:text-white transition-colors"
          >Association for Computing Machinery</a
        >
      </p>
    </div>
  </div>
</footer>

<style global>
  :root {
    /* ACM Blue color palette */
    --color-primary-50: 239 246 255; /* Lightest blue */
    --color-primary-100: 219 234 254;
    --color-primary-200: 191 219 254;
    --color-primary-300: 147 197 253;
    --color-primary-400: 96 165 250;
    --color-primary-500: 59 130 246;
    --color-primary-600: 37 99 235; /* ACM Blue */
    --color-primary-700: 29 78 216;
    --color-primary-800: 30 64 175;
    --color-primary-900: 30 58 138; /* Darkest blue */
  }

  /* Remove focus outline for mouse users but keep it for keyboard navigation */
  *:focus:not(:focus-visible) {
    outline: none;
  }

  *:focus-visible {
    outline: 2px solid var(--color-primary-400);
    outline-offset: 2px;
  }

  /* Smooth scrolling */
  html {
    scroll-behavior: smooth;
  }

  /* Improved contrast for dark mode text */
  .dark {
    --color-text-primary: 226 232 240;
    --color-text-secondary: 203 213 225;
  }

  /* Alert Popup Styles */
  .alert-popup {
    position: fixed;
    top: 20px;
    left: 50%;
    transform: translateX(-50%);
    background-color: rgba(220, 53, 69, 0.9); /* red-ish background */
    color: white;
    padding: 1rem 2rem;
    border-radius: 0.5rem;
    z-index: 1100;
    box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
  }
</style>
