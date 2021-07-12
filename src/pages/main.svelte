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
      // Who am I?
      <span>
        👀 Yeah, who are you?
      </span>
    </h1>
    <div class="about">
      <img src="AboutMePic.jpg" alt="AboutMePic">
      <div class="styled-font">
        <i>"The trajectory of your life bends in the direction of your habits." - James Clear</i>
        <h2>Full Stack Web Developer</h2>
        <p>
          Based in California, currently working as a Software Engineer at <a href="https://modelmatch.com/">Model Match</a>, I started out in Full Stack when the pandemic hit. Shortly after, I enrolled online in UC Irvine's Web Dev bootcamp. Six fast-paced months later, I was well equiped to start a new job and have not looked back since.
        </p>
        <h2>Interactive Developer</h2>
        <p>
          I have a passion for creating interactive experiences. There is a game-like nature of interactive experiences that are very enticing to me. I started using Unity3D for interactives for autoshows back in 2016. That was my first time ever coding professionally. 
        </p>
        <h2>Tinkerer</h2>
        <p>
          Another interest of mine is microcontrollers! I have been dabbling in hardware electronics from a young age. Soldering circuits and getting them to work, brings my imagination and code into a tangable space. The most interesting part of microcontrollers, in my opinion, is getting them to communicate with each other wirelessly and over the internet. Combining this hobby with my main knowledge base of software development, I have been able to improve on my simple projects.
        </p>
        <h2>Streamer</h2>
        <p>
          I have been streaming since 2015 off an on. While I started streaming games back in the day, I found out that coding on streams was more in demand than I expected. Typically, I stream coding to an audience who is not as tech-savvy as I am, generally. Because of that, I need to break down what I am doing into simple english terms. This process helps me reinforce myself and my work.
        </p>
      </div>
    </div>

    <!-- Capabilities Section -->
    <h1 class="styled-font section">
      // Full Stack Capabilities
      <span>
        I am a web developer that loves to try all the things!
      </span>
    </h1>
    <Capabilities/>

    <!-- Projects Section -->
    <h1 class="styled-font section">
      // Recent Projects 
      <span>
        My most recent commits to <a href="https://github.com/ChristianPayne">GitHub</a>.
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
    display: grid;
    grid-template-columns: 1fr 1fr;
    grid-gap: 1.5em;
  }
  .about img {
    border-radius: 0.5em;
    width: 100%;
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

    .about {
      grid-template-columns: 1fr;
    }

    .projects {
      padding-left: 0.5em;
      padding-right: 0.5em;
    }

    .cards {
      grid-template-columns: 1fr;
      font-size: 0.75em;
      grid-gap: 1em;
    }
  }
</style>