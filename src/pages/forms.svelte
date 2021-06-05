<script lang="ts">
  export let name = "World";
</script>

<form name="Test Contact Form" data-netlify="true">
  <p>
    <label>Name <input type="text" name="name" /></label>
  </p>
  <p>
    <label>Email <input type="email" name="email" /></label>
  </p>
  <p>
    <button type="submit">Send</button>
  </p>
</form>

<style>
  h1 {
    margin: 0;
  }
</style>