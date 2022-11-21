<script lang="ts">
  import { Router, Route, Link } from "svelte-routing";
</script>

<Router>
    <!-- Header -->
    <div class="flex justify-between p-4 items-baseline">
      <Link to="/" class="text-4xl">Christian Payne</Link>
      <div class="space-x-4">
        <a href="https://christianpayne.substack.com/">Newsletter</a>
        <a href="https://notes.christianpayne.dev/">Notes</a>
      </div>
    </div>
    <!-- Body -->
    <!-- <Route path="/" component={Main} /> -->
    <!-- Footer -->
    <div class="flex">

    </div>
</Router>