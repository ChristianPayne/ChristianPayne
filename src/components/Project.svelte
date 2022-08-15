<script lang="ts">
  import type { Project } from "../types";

  export let project: Project = null;
</script>

<div class="card">
  <h1>{project.title}</h1>
  <img src={project.image} alt={`Image|${project.title}`}>
  <p>Check it out <a href={project.link}>here!</a></p>
</div>

<style lang="scss">
  @import "../breakpoints.scss";

  .card {
    text-decoration: none;
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    padding-left: 2em;
    padding-right: 2em;
    padding-bottom: 2em;
    border: 1px solid #41444A;
    border-radius: 0.5em;
    transition: background-color 0.3s, font-size 0.3s, border 0.3s;

    &:hover {
      background-color: #41444A;
    }

    img {
      width: 100%;
    }

    h3 {
      margin:0;
    }

    p {
      color: #9D9EA0;
    }

    @include screen-sm-only {
      padding-left: 1em;
      padding-right: 1em;

      h1 {
        font-size: 1.5em;
      }
    }
  }

</style>