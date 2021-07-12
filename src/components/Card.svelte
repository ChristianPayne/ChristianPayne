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

<style>
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
  }

  .card:hover {
    background-color: #41444A;
    
  }

  .card h3 {
    margin:0;
  }

  .card p {
      color: #9D9EA0;
    }

  @media screen and (min-width: 1025px) {
  }
  @media screen and (min-width: 2000px) {
  }
  @media screen and (max-width: 600px) {
    .card {
      padding-left: 1em;
      padding-right: 1em;
    }
    .card h1 {
      font-size: 1.5em;
    }
  }
</style>