<script lang="ts">
  export let onClick: () => void = () => { console.log("On Click not initialized.") }
</script>
<button class="dark:bg-dark-foreground dark:hover:bg-dark-accent bg-light-foreground hover:bg-light-accent px-4 py-2 rounded-sm select-none lowercase transition-all hover:scale-105 active:scale-95" on:click={onClick}>
  <slot>
    Button
  </slot>
</button>