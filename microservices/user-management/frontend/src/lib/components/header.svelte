<script lang="ts">
import { Names } from "$lib/domain/configuration/names";
import { Urls } from "$lib/domain/configuration/urls";

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

  let loggedIn = false

  let isSideNavOpen = false
</script>

<Header href="/" company={Names.SUITE_NAME} platformName={Names.APPLICATION_NAME} bind:isSideNavOpen>
  <div slot="skip-to-content">
    <SkipToContent />
  </div>

  {#if loggedIn}
    <SideNav bind:isOpen={isSideNavOpen} fixed rail>
      <SideNavItems>
        <SideNavLink href="/users" text="Users" icon={Person20} />
      </SideNavItems>
    </SideNav>
  {:else}
    <HeaderNav>
      <Button href="/login" kind="secondary">Log in</Button>
      <Button href="/signup">Sign up</Button>
    </HeaderNav>
  {/if}

  <HeaderUtilities>
    <HeaderActionLink href="/imprint" icon={Attachment20} />
    <HeaderAction>
      <HeaderPanelLinks>
        <HeaderPanelLink>Dashboard</HeaderPanelLink>
        <HeaderPanelDivider>Services</HeaderPanelDivider>
        <HeaderPanelLink href={Urls.BaseUrl}>{Names.APPLICATION_NAME}</HeaderPanelLink>
      </HeaderPanelLinks>
    </HeaderAction>
  </HeaderUtilities>
</Header>
