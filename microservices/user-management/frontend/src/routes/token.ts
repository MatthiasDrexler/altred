import { auth } from "$lib/domain/configuration/auth"
import { AccessTokenDto } from "$lib/dtos/accessTokenDto"
import type { ServerRequest } from "@sveltejs/kit/types/hooks"

type EndpointOutput = { body }

export const post = async (request: ServerRequest): Promise<EndpointOutput> => {
  const code = request.body.get("code")
  const sessionState = request.body.get("session_state")

  console.log(code)
  console.log(sessionState)

  const accessToken = await retrieveTokenForUser(code)
  if (accessToken) {
    return { body: accessToken }
  }
}

const retrieveTokenForUser = async (authorizationCode: string): Promise<AccessTokenDto> => {
  try {
    const response = await fetch(auth.AUTH_URL, {
      method: "POST",
      headers: {
        "Content-Type": "application/x-www-form-urlencoded;charset=UTF-8",
      },
      body: new URLSearchParams({
        grant_type: "authorization_code",
        client_id: auth.AUTH_CLIENT_ID,
        client_secret: auth.AUTH_CLIENT_SECRET,
        code: authorizationCode,
        redirect_uri: auth.AUTH_REDIRECT_URI,
      }),
    })

    if (!response.ok) {
      console.log(await response.text())
      return undefined
    }

    const responseJson = await response.json()

    return new AccessTokenDto(responseJson.token_type, responseJson.access_token)
  } catch (error: unknown) {
    console.error("Could not retrieve token: " + error)
  }
}
