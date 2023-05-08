<script lang="ts">
  type ButtonType = "default" | "simple" | "link"
  let classNames = "";
  export { classNames as class};
  export let onClick: () => void = () => { console.log("On Click not initialized.") }
  export let type: ButtonType = 'default';
  export let linkRef: string = "https://christianpayne.dev/"
</script>

{#if type === 'default'}
  <button class="{classNames} dark:bg-dark-foreground dark:hover:bg-dark-accent bg-light-foreground hover:bg-light-accent px-4 py-2 rounded-sm select-none lowercase transition-all hover:scale-105 active:scale-95 inline-block" on:click={onClick}>
    <slot>
      Default Button
    </slot>
  </button>
{/if}
{#if type === 'simple'}
  <button class="{classNames} select-none lowercase group inline-block" on:click={onClick}>
    <slot>
      Simple Button
    </slot>
    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-4 h-6 inline transition-transform -translate-x-1 group-hover:translate-x-0">
      <path stroke-linecap="round" stroke-linejoin="round" d="M8.25 4.5l7.5 7.5-7.5 7.5" />
    </svg>
  </button>
{/if}
{#if type === 'link'}
  <a class="{classNames} select-none lowercase relative group inline-block" href={linkRef} target="_blank" on:click={onClick}>
    <slot>
      Link Button
    </slot>
    <div class="bg-light-text dark:bg-dark-text absolute bottom-0 left-0 right-full group-hover:right-0 h-[1px] transition-all duration-300"></div>
  </a>
{/if}