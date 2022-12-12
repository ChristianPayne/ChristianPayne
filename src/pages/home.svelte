<script>
  import GitHubRepos from "../components/GitHubRepos.svelte";
  import Button from "../components/Button.svelte";
  import Quote from "../components/Quote.svelte";
</script>

<!-- <img src="Profile_BG_Removed_Medium.png" alt="profile_picture" class="rounded-2xl bg-opacity-20"> -->
<hr/>
<Quote class="text-xl"/>
<hr/>
<p>
  I am a developer who works with code to solve problems. <br/>
  I pride myself on attention to detail and creating strong foundations for others to build off of.
</p>
<p>Based in Southern California, I am currently working as a Developer at <a href="https://modelmatch.com/" class="cursor-pointer underline inline-block" target="_blank">Model Match</a>.</p>
<Button type="default" onClick={() => console.log("JK, we have an onClick.")}></Button>
<Button type="simple" onClick={() => console.log("JK, we have an onClick.")}></Button>
<Button type="link" linkRef="#"></Button>
<h2 class="text-2xl">Case Studies</h2>
<h2 class="text-2xl">Recent commits</h2>
<GitHubRepos></GitHubRepos>