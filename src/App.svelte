<script lang="ts">
  import { Router, Route } from "svelte-routing";
  import Construction from "./pages/construction.svelte";
  import Forms from "./pages/forms.svelte";
  
  export let url = "";

  fetch('./api')
    .then((res)=>{console.log("Here"); return res.json();})
    .then((data)=>{console.log(data)})
</script>

<Router url="{url}">
    <!-- <Route path="blog/:id" component="{BlogPost}" />
    <Route path="blog" component="{Blog}" />-->
    <!-- <Route path="forms/:name" component="{Forms}" /> 
    <Route path="forms" component="{Forms}" />  -->
    <Route path="/"><Construction /></Route>
</Router>