<script>
  import Banner from "../components/Banner.svelte";
  import Capabilities from "../components/Capabilities.svelte";
  import Contact from "../components/Contact.svelte";
  import About from "../components/About.svelte";
  import Projects from "../components/Projects.svelte";
</script>

<div>
  <Banner/>
  <div class="page">
    <!-- About Section -->
    <h1 class="styled-font section">
      // Who am I?
      <span>
        Yeah, who are you?
      </span>
    </h1>
    <About/>

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
    <Projects/>

    <!-- Contact Section -->
    <h1 class="styled-font section">
      // Contact Me
      <span>
        Would you like to talk (or just say hi)?
      </span>
    </h1>
    <Contact/>
  </div>
  <!-- <h1 class="styled-font section">Contact</h1> -->
</div>

<style lang="scss">
  @import "../breakpoints.scss";
  .page {
    padding-left: 2em;
    padding-right: 2em;
    padding-bottom: 2em;
  }

  .section {
    padding-left: 2em;
    border-bottom: #E8EAED solid 1px;
    /* background-color: #6272a4; */
    span {
      color: #9D9EA0;
      font-size: 0.5em;
      margin-left: 1em;
    }

    @include screen-sm-only {
      padding-left: 0.5em;
      font-size: 1.5em;
    }
  }
</style>