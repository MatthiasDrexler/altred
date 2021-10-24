import { auth } from "$lib/domain/configuration/auth"
import { AccessTokenDto } from "$lib/dtos/accessTokenDto"

type EndpointOutput = { body }

export const get = async (): Promise<EndpointOutput> => {
  const token: AccessTokenDto = await retrieveTokenForUser("user", "x")

  if (token) {
    return {
      body: token,
    }
  }
}

const retrieveTokenForUser = async (username: string, password: string): Promise<AccessTokenDto> => {
  try {
    const response = await fetch(auth.AUTH_URL, {
      method: "POST",
      headers: {
        "Content-Type": "application/x-www-form-urlencoded;charset=UTF-8",
      },
      body: new URLSearchParams({
        grant_type: "password",
        client_id: auth.AUTH_CLIENT_ID,
        client_secret: auth.AUTH_CLIENT_SECRET,
        username: username,
        password: password,
      }),
    })

    if (!response.ok) {
      return undefined
    }

    const responseJson = await response.json()

    return new AccessTokenDto(responseJson.token_type, responseJson.access_token)
  } catch (error: unknown) {
    console.error("Could not retrieve token: " + error)
  }
}
