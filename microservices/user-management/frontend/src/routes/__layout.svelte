<script lang="ts">
  import "carbon-components-svelte/css/g100.css"
  import Header from "$lib/Header.svelte"
  import Content from "$lib/Content.svelte"
</script>

<Header />
<Content>
  <slot />
</Content>
