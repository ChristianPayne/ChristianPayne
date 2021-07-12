<script lang="ts">
  import { Router, Route } from "svelte-routing";
  import Construction from "./pages/construction.svelte";
  import Main from "./pages/main.svelte";
  import Forms from "./pages/forms.svelte";
  
  export let url = "";

  async function fetchResults () { 
    // I cant seem to get the proper response from the lambda function.
    // let results = await fetch('./api/hello-world');
    // console.log(results);
    // console.log('Vite changes.');
    
  }

  fetchResults();
  
</script>

<Router url="{url}">
    <!-- <Route path="blog/:id" component="{BlogPost}" />
    <Route path="blog" component="{Blog}" />-->
    <!-- <Route path="forms/:name" component="{Forms}" /> 
    <Route path="forms" component="{Forms}" />  -->
    <Route path="/">
      <!-- <Construction /> -->
      <Main />
    </Route>
</Router>