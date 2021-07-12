<script>
  import { onMount } from "svelte";
  import Card from "../components/Card.svelte";
  import Banner from "../components/Banner.svelte";
  import Capabilities from "../components/Capabilities.svelte";

  
  async function fetchProjects() {
    let response = await fetch('https://api.github.com/users/christianpayne/repos?sort=updated')
    let repoList = await response.json();
    console.log(repoList);
    return repoList.slice(0,6);
  }
  let projects = fetchProjects();
</script>

<div>
  <Banner/>
  <div class="page">
    <!-- About Section -->
    <h1 class="styled-font section">
      Who?
      <span>
        Yeah, that is me.
      </span>
    </h1>
    <div class="about">
      <img src="AboutMePic.jpg" alt="AboutMePic">
    </div>

    <!-- Capabilities Section -->
    <h1 class="styled-font section">
      Capabilities
      <span>
        I am a developer, and I love to learn new things. 
      </span>
    </h1>
    <Capabilities/>

    <!-- Projects Section -->
    <h1 class="styled-font section">
      Recent Projects 
      <span>
        My most recent commits to GitHub.
      </span>
    </h1>
    <div class="projects">
      {#await projects then projects}
      <div class="cards">
        {#each projects as project}
        <Card props={project}/>
        {/each}
      </div>
      {/await}
    </div>
  </div>
  <!-- <h1 class="styled-font section">Contact</h1> -->
</div>

<style>
  .page {
    padding-left: 2em;
    padding-right: 2em;
    padding-bottom: 2em;
    /* padding: 2em; */
  }

  .page i {
    font-size: 1.5em;
  }

  .section {
    padding-left: 2em;
    border-bottom: #E8EAED solid 1px;
    /* background-color: #6272a4; */
  }

  .section span {
    color: #9D9EA0;
    font-size: 0.5em;
    margin-left: 1em;
  }

  .about {
    /* max-height: 50vh; */
    display: flex;
    
    
  }
  .about img {
    width: 50%;
    border-radius: 0.5em;
  }

  .cards {
    display: grid;
    grid-template-columns: 1fr 1fr;
    grid-gap: 1.5em;
  }

  @media screen and (min-width: 1025px) {
  }
  @media screen and (max-width: 600px) {
    .section {
      padding-left: 0.5em;
      font-size: 1.5em;
    }

    .projects {
      padding-left: 0.5em;
      padding-right: 0.5em;
    }

    .cards {
      font-size: 0.75em;
      grid-gap: 1em;
    }
  }
</style>