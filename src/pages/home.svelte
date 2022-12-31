<script lang="ts">
  import GitHubRepos from "../components/GitHubRepos.svelte";
  import Button from "../components/Button.svelte";
  import Quote from "../components/Quote.svelte";

  const codeSnippet: string = `const christian = {
 name = 'Christian Payne'
 role = 'Software Developer'
 phone = <a href="tel:16572341562">'1(657)-234-1562'</a>
 email = <a href="mailto:christianpayne522@gmail.com">'christianpayne522@gmail.com'</a>
 location = 'southern california'
 type = 'Self Starter'
 strengths = [
   'Leadership',
   'Working as a team',
   'Attention to detail'
 ]
}
<a href="tel:16572341562">contact(phone)</a>
<a href="mailto:christianpayne522@gmail.com">contact(email)</a>
`
</script>

<!-- <img src="Profile_BG_Removed_Medium.png" alt="profile_picture" class="rounded-2xl bg-opacity-20"> -->
<hr/>
<Quote class="text-xl"/>
<hr/>
<div class="md:flex space-y-4 md:space-y-0">
  <div class="grow space-y-2">
    <p>I am a developer who works with code to solve problems.</p>
    <p>I pride myself on attention to detail and creating strong foundations for others to build off of.</p>
    <p>Based in Southern California, I am currently working as a Developer at <a href="https://modelmatch.com/" class="cursor-pointer underline inline-block" target="_blank">Model Match</a>.</p>
  </div>
  <pre class="flex-none">
    {@html codeSnippet}
  </pre>
</div>

<Button type="default" onClick={() => console.log("JK, we have an onClick.")}></Button>
<Button type="simple"></Button>
<Button type="link" linkRef="#"></Button>
<h2 class="text-2xl">Case Studies</h2>
<h2 class="text-2xl">Recent commits</h2>
<GitHubRepos></GitHubRepos>