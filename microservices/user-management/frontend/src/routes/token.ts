import { urls } from "$lib/domain/configuration/public/urls"
import { AuthorizationService } from "$lib/domain/services/authorization/authorizationService"
import type { RequestHandler } from "@sveltejs/kit"

type EndpointOutput = { headers; status }

export const post: RequestHandler<unknown, FormData> = async ({ body }): Promise<EndpointOutput> => {
  const code = body.get("code")
  const sessionState = body.get("session_state")

  console.log("Authorization Code: " + code)
  console.log("Session State: " + sessionState)

  await new AuthorizationService().retrieveTokenForUser(code)

  return {
    headers: { Location: urls.ENTRY_URL },
    status: 302,
  }
}
