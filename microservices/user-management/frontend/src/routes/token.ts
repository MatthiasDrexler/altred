import { AuthorizationService } from "$lib/domain/services/authorization/authorizationService"
import type { ServerRequest } from "@sveltejs/kit/types/hooks"

type EndpointOutput = { body }

export const post = async (request: ServerRequest): Promise<EndpointOutput> => {
  const code = request.body.get("code")
  const sessionState = request.body.get("session_state")

  console.log(code)
  console.log(sessionState)

  const accessToken = await new AuthorizationService().retrieveTokenForUser(code)
  if (accessToken) {
    return { body: accessToken }
  }
}
