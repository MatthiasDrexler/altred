import { auth } from "$lib/domain/configuration/private/auth"
import { AccessTokenDto } from "$lib/dtos/accessTokenDto"

class AuthenticationService {
  constructor(private _sessionState: string = undefined) {}

  public isUnauthenticated(): boolean {
    return this._sessionState == undefined
  }

  retrieveTokenForUser = async (authorizationCode: string): Promise<AccessTokenDto> => {
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

      this._sessionState = responseJson.session_state

      return new AccessTokenDto(responseJson.token_type, responseJson.access_token)
    } catch (error: unknown) {
      console.error("Could not retrieve token: " + error)
    }
  }
}

export const authenticationService = new AuthenticationService()