<script lang="ts">
  import "carbon-components-svelte/css/g100.css"
  import Header from "$lib/components/header.svelte"
  import Content from "$lib/components/content.svelte"
</script>

<Header />
<Content>
  <slot />
</Content>
