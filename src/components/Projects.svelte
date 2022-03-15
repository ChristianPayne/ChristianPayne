<script lang="ts">
  import type { Project as ProjectType } from "../types";
  import Project from "./Project.svelte";
  

  let projects: ProjectType[] = [
    {
      title: 'The Event Community',
      image: 'https://firebasestorage.googleapis.com/v0/b/christianpayne-dev.appspot.com/o/Makhsies.jpg?alt=media&token=4f60fe4f-5e2f-4d36-b633-168c0b224eba',
      link: 'https://github.com/BAVEL-Technology/event-community'
    },
    {
      title: 'Frank The Plumber',
      image: 'https://firebasestorage.googleapis.com/v0/b/christianpayne-dev.appspot.com/o/Screen%20Shot%202022-03-14%20at%209.58.54%20PM.png?alt=media&token=8de7f66f-fc6b-4d09-8180-a9437bba9e2b',
      link: 'https://github.com/ChristianPayne/franktheplumber'
    },
  ]
</script>

<div class="styled-font cards">
  {#each projects as project}
    <Project project={project}/>
  {/each}
</div>

<style lang="scss">
  @import "../breakpoints.scss";

  .cards {
    display: grid;
    font-size: 0.75em;

    @include screen-sm-only {
      grid-template-columns: 1fr;
      grid-gap: 1em;
    }

    @include screen-md-up {
      grid-template-columns: 1fr 1fr;
      grid-gap: 1.5em;
    }
    
    @include screen-lg-up {
      font-size: 1em;
    }
    
  }
</style>