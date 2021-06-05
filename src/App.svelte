<script lang="ts">
  import { Router, Route } from "svelte-routing";
  import Construction from "./pages/construction.svelte";
  import Forms from "./pages/forms.svelte";
  
  export let url = "";
</script>

<Router url="{url}">
    <!-- <Route path="blog/:id" component="{BlogPost}" />
    <Route path="blog" component="{Blog}" />-->
    <Route path="forms/:name" component="{Forms}" /> 
    <Route path="forms" component="{Forms}" /> 
    <Route path="/"><Construction /></Route>
</Router>