<script lang="ts">
  import { Column, Grid, Row } from "carbon-components-svelte"
  import { page } from "$app/stores"

  const sessionState = $page.query.get("session_state")
  const code = $page.query.get("code")

  // async function getTokenFromEndpoint() {
  //   const response = await fetch("/token")
  //   if (!response.ok) {
  //     throw Error("Could not retrieve token: " + response.statusText)
  //   }
  //   const responseJson = await response.json()

  //   return responseJson
  // }
</script>

<Grid>
  <Row>
    <Column>
      <h1>Log in</h1>

      Session State: {sessionState}<br />
      Code: {code}<br />

      <!-- {#await tokenPromise}
        Load token
      {:then token}
        Token: {token.accessToken}
      {/await} -->
    </Column>
  </Row>
</Grid>
