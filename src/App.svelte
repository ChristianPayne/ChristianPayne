<script lang="ts">
  import { Router, Route, Link } from "svelte-routing";
  import Home from "./pages/home.svelte";

  let darkMode = false;
</script>

<style>
  :global(body) {
    @apply bg-black;
  }
</style>

<div class={darkMode ? "dark" : "light"}>
  <div class="dark:bg-black dark:text-white bg-white text-black h-screen p-2 sm:p-4 transition-colors duration-1000">
    <Router>
        <!-- Header -->
        <div class="sm:flex sm:justify-between items-center mb-4">
          <Link to="/" class="text-4xl font-medium text-center sm:text-left">Christian Payne</Link>
          <div class="text-center space-x-4">
            <a class="h-5" href="https://christianpayne.substack.com/" target="_blank">Newsletter</a>
            <a class="h-5" href="https://notes.christianpayne.dev/" target="_blank">Notes</a>
            <span on:click={() => darkMode = !darkMode} class="cursor-pointer inline-block h-5">
              {#if darkMode}
                <!-- Moon -->
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
                  <path stroke-linecap="round" stroke-linejoin="round" d="M21.752 15.002A9.718 9.718 0 0118 15.75c-5.385 0-9.75-4.365-9.75-9.75 0-1.33.266-2.597.748-3.752A9.753 9.753 0 003 11.25C3 16.635 7.365 21 12.75 21a9.753 9.753 0 009.002-5.998z" />
                </svg>
              {:else}
                <!-- Sun -->
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
                  <path stroke-linecap="round" stroke-linejoin="round" d="M12 3v2.25m6.364.386l-1.591 1.591M21 12h-2.25m-.386 6.364l-1.591-1.591M12 18.75V21m-4.773-4.227l-1.591 1.591M5.25 12H3m4.227-4.773L5.636 5.636M15.75 12a3.75 3.75 0 11-7.5 0 3.75 3.75 0 017.5 0z" />
                </svg>
              {/if}
            </span>
          </div>
        </div>
        <!-- Body -->
        <Route path="/" component={Home} />
        <!-- Footer -->
        <div class="flex">
  
        </div>
    </Router>
  </div>
</div>