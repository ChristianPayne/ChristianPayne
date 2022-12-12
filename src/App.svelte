<script lang="ts">
  import { Router, Route, Link } from "svelte-routing";
  import Home from "./pages/home.svelte";
  import Button from "./components/Button.svelte";

  let darkMode = false;

  $: {
    let body = document.querySelector('body');

    if(darkMode) {
      body.style.backgroundColor = '#141414';
    } else {
      body.style.backgroundColor = '#F5F5F5';
    }
  }
</script>

<style>
  :global(body) {
    @apply transition-colors duration-500 lowercase;
    font-family: 'Alexandria', sans-serif;
  }
</style>

<div class={darkMode ? "dark" : "light"}>
  <div class="dark:bg-dark-background dark:text-dark-text bg-light-background text-light-text p-2 sm:p-4 transition-colors duration-500 space-y-4">
    <Router>
        <!-- Header -->
        <header class="sm:flex sm:justify-between items-center mb-4">
          <Link to="/" class="text-4xl font-medium text-center sm:text-left block mb-4 sm:m-0">Christian Payne</Link>
          <div class="text-center space-x-4 flex justify-between">
            <div class="w-6 h-6"></div>
            <div class="space-x-2">
              <Button style='link' linkRef="https://christianpayne.substack.com/">Newsletter</Button>
              <Button style='link' linkRef="https://christianpayne.substack.com/">Notes</Button>
            </div>
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
        </header>
        <!-- Body -->
        <main class="pb-8 space-y-4">
          <Route path="/" component={Home} />
        </main>
        <!-- Footer -->
        <footer class="flex space-x-4 [&>*]:h-6 transition-colors duration-500 dark:bg-dark-background dark:text-dark-text bg-light-background text-light-text fixed bottom-0 left-0 right-0 p-2">
          <p>Christian Payne</p>
          <a href="https://www.github.com/christianpayne/" target="_blank">GitHub</a>
          <a href="https://www.linkedin.com/in/christianpayne522/" target="_blank">LinkedIn</a>
        </footer>
    </Router>
  </div>
</div>