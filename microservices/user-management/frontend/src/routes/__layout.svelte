<script lang="ts" context="module">
  // eslint-disable-next-line @typescript-eslint/explicit-module-boundary-types
  export const load = async ({ fetch }): Promise<{ props }> => {
    const fetchedNames = await fetch("names")
    const names = await fetchedNames.json()

    const fetchedUrls = await fetch("urls")
    const urls = await fetchedUrls.json()

    return {
      props: {
        names,
        urls,
      },
    }
  }
</script>

<script lang="ts">
  import "carbon-components-svelte/css/g100.css"
  import { Content } from "carbon-components-svelte"
  import {
    Button,
    Header,
    HeaderAction,
    HeaderActionLink,
    HeaderNav,
    HeaderPanelDivider,
    HeaderPanelLink,
    HeaderPanelLinks,
    HeaderUtilities,
    SideNav,
    SideNavItems,
    SideNavLink,
    SkipToContent,
  } from "carbon-components-svelte"
  import Attachment20 from "carbon-icons-svelte/lib/Attachment20"
  import Person20 from "carbon-icons-svelte/lib/Person20"

  export let names
  export let urls

  let unauthenticated = true
  let isSideNavOpen = false
</script>

<Header href="/" company={names.SUITE_NAME} platformName={names.APPLICATION_NAME} bind:isSideNavOpen>
  <div slot="skip-to-content">
    <SkipToContent />
  </div>

  {#if unauthenticated}
    <HeaderNav>
      <Button href={urls.LOGIN_URL} kind="secondary">Log in</Button>
      <Button href="/signup">Sign up</Button>
    </HeaderNav>
  {:else}
    <SideNav bind:isOpen={isSideNavOpen} fixed rail>
      <SideNavItems>
        <SideNavLink href="/users" text="Users" icon={Person20} />
      </SideNavItems>
    </SideNav>
  {/if}

  <HeaderUtilities>
    <HeaderActionLink href="/imprint" icon={Attachment20} />
    <HeaderAction>
      <HeaderPanelLinks>
        {#if unauthenticated}
          <HeaderPanelLink href={urls.LOGIN_URL}>Log in</HeaderPanelLink>
          <HeaderPanelLink href="/signup">Sign up</HeaderPanelLink>
          <HeaderPanelDivider />
        {/if}
        <HeaderPanelLink>Dashboard</HeaderPanelLink>
        <HeaderPanelDivider>Services</HeaderPanelDivider>
        <HeaderPanelLink href={urls.BASE_URL}>{names.APPLICATION_NAME}</HeaderPanelLink>
      </HeaderPanelLinks>
    </HeaderAction>
  </HeaderUtilities>
</Header>

<Content>
  <slot />
</Content>
