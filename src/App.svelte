<script lang="ts">
  import "./lib/TailwindCSS.svelte";
  import { onMount } from "svelte";
  import { Route, Router, navigate } from "svelte-routing";
  import { stores } from "./lib/database/Store";
  import HomePage from "./pages/home/HomePage.svelte";
  import SetupPage from "./pages/setup/SetupPage.svelte";

  let url = "loading";

  onMount(async () => {
    const minecraftPath = await stores.settings.get("minecraftPath");
    if (!minecraftPath) {
      navigate("/setup");
    } else {
      navigate("/");
    }
  });
</script>

<main class="container">
  <Router {url}>
    <Route path="loading">Loading</Route>
    <Route path="/" component={HomePage} />
    <Route path="/setup" component={SetupPage} />
  </Router>
</main>
