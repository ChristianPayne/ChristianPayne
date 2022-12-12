<script>
  async function fetchRepos() {
    let response = await fetch('https://api.github.com/users/christianpayne/repos?sort=updated')
    let repoList = await response.json();
    // console.log(repoList);
    return repoList.slice(0,6).map(repo => {
      let date = new Date(repo.updated_at)
      return {
        ...repo,
        formattedDate: (date.getMonth() + 1) + '/' + date.getDate() + '/' +  date.getFullYear()
      }
    });
  }
  let repos = fetchRepos();
</script>
<div class="divide-y">
  {#await repos then repos}
    {#each repos as repo}
    <!-- Name | Desc | Last Updated -->
      <a class="inline-block group" href="{repo.html_url}" target="_blank">
        <span>{repo.name}</span>
        {#if repo.description}
          <span class="dark:text-dark-accent text-light-accent dark:group-hover:text-dark-text group-hover:text-light-text">| {repo.description}</span>
        {/if}
        <!-- let date = props.updated_at ? new Date(props.updated_at) : new Date(); -->
        <span class="dark:text-dark-accent text-light-accent">{ repo.formattedDate}</span>
      </a>
      <!-- date.getMonth() + 1) + '/' + date.getDate() + '/' +  date.getFullYear() -->
      <p></p>
      <!-- {JSON.stringify(repo)} -->
    {/each}
  {/await}
</div>