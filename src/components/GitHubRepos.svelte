<script>
  import Card from "./Card.svelte";

  async function fetchRepos() {
    let response = await fetch('https://api.github.com/users/christianpayne/repos?sort=updated')
    let repoList = await response.json();
    console.log(repoList);
    return repoList.slice(0,6);
  }
  let repos = fetchRepos();
</script>
<div class="projects">
  {#await repos then repos}
  <div class="cards">
    {#each repos as repo}
      <Card props={repo}/>
    {/each}
  </div>
  {/await}
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