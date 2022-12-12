<script>
  import Button from "./Button.svelte";
  async function fetchRepos() {
    let response = await fetch('https://api.github.com/users/christianpayne/repos?sort=updated')
    let repoList = await response.json();
    return repoList.slice(0,6).map(repo => {
      let date = new Date(repo.updated_at)
      // console.log(repo);
      return {
        ...repo,
        formattedDate: (date.getMonth() + 1) + '/' + date.getDate() + '/' +  date.getFullYear()
      }
    });
  }
  let repos = fetchRepos();
</script>
<div>
  {#await repos then repos}
    {#each repos as repo}
    <!-- Name | Desc | Last Updated -->
      <Button type="link" class="group" linkRef="{repo.html_url}">
        <span>{repo.name}</span>
        {#if repo.description}
          <span class="dark:text-dark-accent text-light-accent dark:group-hover:text-dark-text group-hover:text-light-text">| {repo.description}</span>
        {/if}
        <!-- let date = props.updated_at ? new Date(props.updated_at) : new Date(); -->
      </Button>
      <span class="dark:text-dark-accent text-light-accent">{ repo.formattedDate}</span>
      <br/>
    {/each}
  {/await}
</div>