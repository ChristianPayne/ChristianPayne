<script>
  export let props = {};

  console.log(`Props | ${props}`);

  let date = props.updated_at ? new Date(props.updated_at) : new Date();
</script>

<a class="card " href="{props.html_url}" target="_blank">
  <h1 class="styled-font">: {props.name} :</h1>
  {#if props.description}
    <h3 class="styled-font">{props.description}</h3>
  {/if}
  <p class="styled-font">Last Updated: { date.toDateString()}</p>
</a>

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
    border: 1px solid #41444A;
    border-radius: 0.5em;
    transition: background-color 0.3s, font-size 0.3s, border 0.3s;

    &:hover {
      background-color: #41444A;
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